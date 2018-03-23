//! DO NOT EDIT
//! This file is automatically generated by the varlink rust generator

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::io;

use varlink;
use serde_json;
use varlink::CallTrait;

#[derive(Serialize, Deserialize, Debug)]
struct _PingReply {
    #[serde(skip_serializing_if = "Option::is_none")] pong: Option<String>,
}

impl varlink::VarlinkReply for _PingReply {}

#[derive(Serialize, Deserialize, Debug)]
struct _PingArgs {
    #[serde(skip_serializing_if = "Option::is_none")] ping: Option<String>,
}

pub trait _CallErr: varlink::CallTrait {}

impl<'a> _CallErr for varlink::Call<'a> {}

pub trait _CallPing: _CallErr {
    fn reply(&mut self, pong: Option<String>) -> io::Result<()> {
        self.reply_struct(_PingReply { pong }.into())
    }
}

impl<'a> _CallPing for varlink::Call<'a> {}

pub trait VarlinkInterface {
    fn ping(&self, call: &mut _CallPing, ping: Option<String>) -> io::Result<()>;
    fn call_upgraded(&self, _call: &mut varlink::Call) -> io::Result<()> {
        Ok(())
    }
}

pub struct _InterfaceProxy {
    inner: Box<VarlinkInterface + Send + Sync>,
}

pub fn new(inner: Box<VarlinkInterface + Send + Sync>) -> _InterfaceProxy {
    _InterfaceProxy { inner }
}

impl varlink::Interface for _InterfaceProxy {
    fn get_description(&self) -> &'static str {
        r#"
# Example service
interface org.example.ping

# Returns the same string
method Ping(ping: string) -> (pong: string)


"#
    }

    fn get_name(&self) -> &'static str {
        "org.example.ping"
    }

    fn call_upgraded(&self, call: &mut varlink::Call) -> io::Result<()> {
        self.inner.call_upgraded(call)
    }

    fn call(&self, call: &mut varlink::Call) -> io::Result<()> {
        let req = call.request.unwrap();
        let method = req.method.clone();
        match method.as_ref() {
            "org.example.ping.Ping" => {
                if let Some(args) = req.parameters.clone() {
                    let args: _PingArgs = serde_json::from_value(args)?;
                    return self.inner.ping(call as &mut _CallPing, args.ping);
                } else {
                    return call.reply_invalid_parameter(None);
                }
            }

            m => {
                let method: String = m.clone().into();
                return call.reply_method_not_found(Some(method));
            }
        }
    }
}
