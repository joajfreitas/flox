use crate::source_info::SourceInfo;
use crate::stage2::{S2, T2};
use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;
use std::sync::RwLock;

#[derive(Debug, Clone)]
pub enum T3 {
    Nil,
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    Sym(String),
    List(Vec<S3>),
    Do(Vec<S3>),
    Lambda(Vec<S3>, Box<S3>),
    Defun(Box<S3>, Vec<S3>, Box<S3>),
    If(Box<S3>, Box<S3>, Box<S3>),
    Set(Box<S3>, Box<S3>),
    Func(fn(Vec<S3>) -> Result<S3, String>),
    T3Func {
        eval: fn(ast: Arc<S3>, env: Env) -> Result<S3, String>,
        ast: Arc<S3>,
        env: Env,
        params: Arc<S3>,
    },
}

impl fmt::Display for T3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                T3::Nil => "nil".to_string(),
                T3::Bool(true) => "true".to_string(),
                T3::Bool(false) => "false".to_string(),
                T3::Int(i) => format!("{}", i),
                T3::Float(f) => format!("{}", f),
                T3::Str(s) => s.to_string(),
                T3::Sym(s) => s.to_string(),
                T3::List(ls) => format!("{}", S3s(ls.clone())),
                T3::Do(xs) => format!("({})", S3s(xs.clone())),
                T3::Lambda(args, body) => format!("( lambda {} {} )", S3s(args.clone()), body),
                T3::Defun(name, args, body) =>
                    format!("( defun {} {} {} )", name, S3s(args.clone()), body),
                T3::If(_, _, _) => "if".to_string(),
                T3::Set(_, _) => "set".to_string(),
                T3::T3Func { .. } => "(func)".to_string(),
                T3::Func(_) => "(func)".to_string(),
                _ => panic!(),
            }
        )
    }
}

#[derive(Debug, Clone)]
pub struct S3 {
    t3: T3,
    source_info: SourceInfo,
}

impl fmt::Display for S3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_type())
    }
}

struct S3s(pub Vec<S3>);
impl fmt::Display for S3s {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "( {} )",
            self.0
                .iter()
                .map(|x| format!("{}", x))
                .intersperse(" ".to_string())
                .collect::<String>()
        )
    }
}

impl S3 {
    fn new(t3: &T3, source_info: &SourceInfo) -> S3 {
        S3 {
            t3: t3.clone(),
            source_info: source_info.clone(),
        }
    }
    pub fn get_type(&self) -> &T3 {
        &self.t3
    }

    pub fn nil(source_info: &SourceInfo) -> S3 {
        S3::new(&T3::Nil, source_info)
    }

    pub fn bool(b: bool, source_info: &SourceInfo) -> S3 {
        S3::new(&T3::Bool(b), source_info)
    }

    pub fn int(i: i64, source_info: &SourceInfo) -> S3 {
        S3::new(&T3::Int(i), source_info)
    }

    pub fn float(f: f64, source_info: &SourceInfo) -> S3 {
        S3::new(&T3::Float(f), source_info)
    }

    pub fn str(s: &str, source_info: &SourceInfo) -> S3 {
        S3::new(&T3::Str(s.to_string()), source_info)
    }

    pub fn sym(s: &str, source_info: &SourceInfo) -> S3 {
        S3::new(&T3::Sym(s.to_string()), source_info)
    }

    pub fn list(l: Vec<S3>, source_info: &SourceInfo) -> S3 {
        S3::new(&T3::List(l), source_info)
    }

    pub fn do_(l: Vec<S3>, source_info: &SourceInfo) -> S3 {
        S3::new(&T3::Do(l), source_info)
    }

    pub fn lambda(args: Vec<S3>, body: Box<S3>, source_info: &SourceInfo) -> S3 {
        S3::new(&T3::Lambda(args, body), source_info)
    }

    pub fn defun(name: Box<S3>, args: Vec<S3>, body: Box<S3>, source_info: &SourceInfo) -> S3 {
        S3::new(&T3::Defun(name, args, body), source_info)
    }

    pub fn if_(
        pred: Box<S3>,
        cond_true: Box<S3>,
        cond_false: Box<S3>,
        source_info: &SourceInfo,
    ) -> S3 {
        S3::new(&T3::If(pred, cond_true, cond_false), source_info)
    }

    pub fn set(lvalue: Box<S3>, rvalue: Box<S3>, source_info: &SourceInfo) -> S3 {
        S3::new(&T3::Set(lvalue, rvalue), source_info)
    }

    pub fn get_int(&self) -> i64 {
        match self.get_type() {
            T3::Int(i) => *i,
            _ => panic!(),
        }
    }

    pub fn get_sym(&self) -> Option<&str> {
        match self.get_type() {
            T3::Sym(s) => Some(s),
            _ => None,
        }
    }

    pub fn get_list(&self) -> Vec<S3> {
        match self.get_type() {
            T3::List(l) => l.clone(),
            _ => panic!(),
        }
    }

    pub fn parse(s2: &S2) -> Result<S3, String> {
        Ok(match &s2.get_type() {
            T2::Nil => S3::nil(&s2.source_info),
            T2::Bool(b) => S3::bool(*b, &s2.source_info),
            T2::Int(i) => S3::int(*i, &s2.source_info),
            T2::Float(f) => S3::float(*f, &s2.source_info),
            T2::Str(s) => S3::str(&s, &s2.source_info),
            T2::Sym(s) => S3::sym(&s, &s2.source_info),
            T2::List(l) => S3::list(
                l.iter()
                    .map(|s2| S3::parse(s2).unwrap())
                    .collect::<Vec<S3>>(),
                &s2.source_info,
            ),
            T2::Do(do_) => S3::do_(
                do_.iter()
                    .map(|s2| S3::parse(s2).unwrap())
                    .collect::<Vec<S3>>(),
                &s2.source_info,
            ),
            T2::Lambda(args, body) => S3::lambda(
                args.iter()
                    .map(|s2| S3::parse(s2).unwrap())
                    .collect::<Vec<S3>>(),
                Box::new(S3::parse(body)?),
                &s2.source_info,
            ),
            T2::Defun(name, args, body) => S3::defun(
                Box::new(S3::parse(name)?),
                args.iter()
                    .map(|s2| S3::parse(s2).unwrap())
                    .collect::<Vec<S3>>(),
                Box::new(S3::parse(body)?),
                &s2.source_info,
            ),
            T2::If(pred, cond_true, cond_false) => S3::if_(
                Box::new(S3::parse(pred)?),
                Box::new(S3::parse(cond_true)?),
                Box::new(S3::parse(cond_false)?),
                &s2.source_info,
            ),
            T2::Set(lvalue, rvalue) => S3::set(
                Box::new(S3::parse(lvalue)?),
                Box::new(S3::parse(rvalue)?),
                &s2.source_info,
            ),
            _ => panic!(),
        })
    }

    pub fn eval(s3: &S3) -> Result<S3, String> {
        let env = env_new(None);
        std_lib(&env);
        S3::eval_internal(s3, &env)
    }

    fn eval_internal(s3: &S3, env: &Env) -> Result<S3, String> {
        match &s3.get_type() {
            T3::Int(_) | T3::Float(_) | T3::Str(_) | T3::Bool(_) => Ok(s3.clone()),
            T3::List(l) => S3::eval_list(l, env),
            T3::Set(lvalue, rvalue) => S3::eval_set(lvalue, rvalue, env),
            T3::Sym(sym) => S3::eval_sym(s3, env),
            T3::Do(stmts) => S3::eval_do(stmts, env),
            T3::If(pred, cond_true, cond_false) => S3::eval_if(pred, cond_true, cond_false, env),
            T3::Lambda(args, body) => S3::eval_lambda(args, body, env),
            _ => panic!(),
        }
    }

    fn eval_set(lvalue: &S3, rvalue: &S3, env: &Env) -> Result<S3, String> {
        let r = S3::eval_internal(rvalue, &env_new(Some(env.clone())))?;
        env_set(env, lvalue.get_sym().unwrap().to_string(), r.clone())?;
        Ok(r)
    }

    fn eval_list(l: &Vec<S3>, env: &Env) -> Result<S3, String> {
        if l[0].get_sym().is_none() || env_get(env, l[0].get_sym().unwrap().to_string()).is_err() {
            Ok(S3::list(
                l.iter()
                    .map(|x| S3::eval_internal(x, env).unwrap())
                    .collect::<Vec<S3>>(),
                &l[0].source_info,
            ))
        } else {
            match env_get(env, l[0].get_sym().unwrap().to_string())
                .unwrap()
                .get_type()
            {
                T3::T3Func {
                    eval,
                    ast,
                    env: menv,
                    params,
                    ..
                } => {
                    let env = env_bind(
                        Some(menv.clone()),
                        params.clone(),
                        l[1..]
                            .iter()
                            .map(|arg| S3::eval_internal(arg, env).unwrap())
                            .collect::<Vec<S3>>(),
                    )
                    .unwrap();
                    eval(ast.clone(), env)
                }
                T3::Func(closure) => closure(
                    l[1..]
                        .iter()
                        .map(|arg| S3::eval_internal(arg, env).unwrap())
                        .collect::<Vec<S3>>(),
                ),
                _ => panic!(),
            }
        }
    }

    fn eval_sym(sym: &S3, env: &Env) -> Result<S3, String> {
        let symbol = sym.get_sym().ok_or("Failed to get sym".to_string())?;
        Ok(match env_get(env, symbol.to_string()) {
            Ok(value) => value,
            Err(_) => match &env.outer {
                Some(env) => S3::eval_sym(sym, env)?,
                //None => sym.clone(),
                None => Err("failed to find symbol")?,
            },
        })
    }

    fn eval_do(stmts: &[S3], env: &Env) -> Result<S3, String> {
        Ok(stmts
            .iter()
            .map(|stmt| S3::eval_internal(stmt, env).unwrap())
            .collect::<Vec<S3>>()
            .pop()
            .unwrap())
    }

    fn eval_if(pred: &S3, cond_true: &S3, cond_false: &S3, env: &Env) -> Result<S3, String> {
        let pred = S3::eval_internal(pred, env)?;
        match pred.get_type() {
            T3::Bool(true) => S3::eval_internal(cond_true, env),
            T3::Bool(false) => S3::eval_internal(cond_false, env),
            _ => Err("Expected boolean".to_string()),
        }
    }

    fn eval_lambda(args: &Vec<S3>, body: &S3, env: &Env) -> Result<S3, String> {
        let env = env_new(Some(env.clone()));
        Ok(S3::new(
            &T3::T3Func {
                eval: eval,
                ast: Arc::new(body.clone()),
                env: env.clone(),
                params: Arc::new(S3::list(args.clone(), &SourceInfo::default())),
            },
            &SourceInfo::default(),
        ))
    }
}

#[derive(Debug)]
pub struct EnvStruct {
    data: RwLock<HashMap<String, S3>>,
    outer: Option<Env>,
}

type Env = Arc<EnvStruct>;

fn env_new(outer: Option<Env>) -> Env {
    Arc::new(EnvStruct {
        data: RwLock::new(HashMap::default()),
        outer,
    })
}

fn env_set(env: &Env, key: String, value: S3) -> Result<(), String> {
    env.data.write().unwrap().insert(key, value);
    Ok(())
}

fn env_keys(env: &Env) -> Vec<(String, String)> {
    let mut outer_keys = if env.outer.is_some() {
        env_keys(&env.outer.clone().unwrap())
    } else {
        vec![]
    };
    outer_keys.append(
        &mut env
            .data
            .read()
            .unwrap()
            .iter()
            .map(|(key, value)| (key.to_string(), format!("{}", value)))
            .collect::<Vec<(String, String)>>(),
    );
    outer_keys
}

fn env_get(env: &Env, key: String) -> Result<S3, String> {
    match env.data.read().unwrap().get(&key) {
        Some(value) => Ok(value.clone()),
        None => {
            if env.outer.is_some() {
                env_get(env.outer.as_ref().unwrap(), key)
            } else {
                Err("failed to find symbol".to_string())
            }
        }
    }
}

pub fn env_bind(outer: Option<Env>, mbinds: Arc<S3>, exprs: Vec<S3>) -> Result<Env, String> {
    let env = env_new(outer);
    match mbinds.clone().get_type() {
        T3::List(binds) => {
            for (i, b) in binds.iter().enumerate() {
                match b.get_type() {
                    T3::Sym(s) if s == "&" => {
                        env_set(
                            &env,
                            binds[i + 1].clone().get_sym().unwrap().to_string(),
                            S3::list(exprs[i..].to_vec(), &SourceInfo::default()),
                        )?;
                        break;
                    }
                    _ => {
                        env_set(
                            &env,
                            b.clone().get_sym().unwrap().to_string(),
                            exprs[i].clone(),
                        )?;
                    }
                }
            }
            Ok(env)
        }
        _ => Err("env bind binds not List/Vector".to_string()),
    }
}

fn eval(ast: Arc<S3>, env: Env) -> Result<S3, String> {
    S3::eval_internal(&*ast, &env)
}

fn std_lib(env: &Env) {
    env_set(
        env,
        "+".to_string(),
        S3::new(
            &T3::Func(|args: Vec<S3>| {
                Ok(S3::int(
                    args.iter().map(|x| x.get_int()).sum(),
                    &SourceInfo::default(),
                ))
            }),
            &SourceInfo::default(),
        ),
    );
}
