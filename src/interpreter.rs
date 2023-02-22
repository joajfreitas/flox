use crate::stage2::{S2, T2};
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;

#[derive(Debug, Clone)]
pub enum Val {
    Nil,
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    Sym(String),
    List(Vec<Val>),
    Func(fn(Vec<Val>) -> Result<Val, String>),
}

impl Val {
    pub fn nil() -> Val {
        Val::Nil
    }

    pub fn bool(b: bool) -> Val {
        Val::Bool(b)
    }

    pub fn int(i: i64) -> Val {
        Val::Int(i)
    }

    pub fn float(f: f64) -> Val {
        Val::Float(f)
    }

    pub fn str(s: &str) -> Val {
        Val::Str(s.to_string())
    }

    pub fn sym(s: &str) -> Val {
        Val::Sym(s.to_string())
    }

    pub fn list(l: Vec<Val>) -> Val {
        Val::List(l)
    }
}

pub struct Interpreter {}

#[derive(Debug)]
struct EnvStruct {
    data: RwLock<HashMap<String, Val>>,
    outer: Option<Env>,
}

type Env = Arc<EnvStruct>;

fn env_new(outer: Option<Env>) -> Env {
    Arc::new(EnvStruct {
        data: RwLock::new(HashMap::default()),
        outer: outer,
    })
}

fn env_set(env: &Env, key: String, value: Val) -> Result<(), String> {
    env.data.write().unwrap().insert(key, value);
    Ok(())
}

fn env_get(env: &Env, key: String) -> Result<Val, String> {
    Ok(env.data.read().unwrap().get(&key).unwrap().clone())
}

impl Interpreter {
    pub fn interpret(s2: S2) -> Result<Val, String> {
        let env = env_new(None);
        let result = Interpreter::eval(&s2, &env.clone());
        println!("{:?}", env.data);
        result
    }

    fn eval(s2: &S2, env: &Env) -> Result<Val, String> {
        match &s2.t2 {
            T2::Int(i) => Ok(Val::int(*i)),
            T2::Float(f) => Ok(Val::float(*f)),
            T2::Str(s) => Ok(Val::str(s)),
            T2::Bool(b) => Ok(Val::bool(*b)),
            T2::List(l) => Ok(Val::list(
                l.iter()
                    .map(|x| Interpreter::eval(x, env).unwrap())
                    .collect::<Vec<Val>>(),
            )),
            T2::Sym(_) => Interpreter::eval_sym(s2.clone(), env),
            T2::Do(stmts) => Interpreter::eval_do(&stmts, env),
            T2::If(pred, cond_true, cond_false) => {
                Interpreter::eval_if(pred, cond_true, cond_false, env)
            }
            T2::Set(lvalue, rvalue) => Interpreter::eval_set(lvalue, rvalue, env),
            _ => panic!(),
        }
    }

    fn eval_sym(sym: S2, env: &Env) -> Result<Val, String> {
        let symbol = sym.get_sym().ok_or("Failed to get sym".to_string())?;
        Ok(match env_get(env, symbol.clone()) {
            Ok(value) => value,
            Err(_) => Val::sym(&symbol),
        })
    }

    fn eval_do(stmts: &Vec<S2>, env: &Env) -> Result<Val, String> {
        Ok(stmts
            .iter()
            .map(|stmt| Interpreter::eval(stmt, env).unwrap())
            .collect::<Vec<Val>>()
            .pop()
            .unwrap())
    }

    //fn eval_lambda(args: Vec<S2>, body: &S2, env: &Env) {
    //    let env = env_new(Some(env.clone()));
    //    Val::Func(|args| Interpreter::eval(body, &env))
    //}

    fn eval_if(pred: &S2, cond_true: &S2, cond_false: &S2, env: &Env) -> Result<Val, String> {
        let pred = Interpreter::eval(pred, env)?;
        match pred {
            Val::Bool(true) => Interpreter::eval(cond_true, env),
            Val::Bool(false) => Interpreter::eval(cond_false, env),
            _ => Err("Expected boolean".to_string()),
        }
    }

    fn eval_set(lvalue: &S2, rvalue: &S2, env: &Env) -> Result<Val, String> {
        let r = Interpreter::eval(rvalue, &env_new(Some(env.clone())))?;
        env_set(&env, lvalue.get_sym().unwrap(), r.clone())?;
        Ok(r)
    }
}
