use anyhow::anyhow;
use clap::{AppSettings, Parser};
use colored::*;
use mime::Mime;
use reqwest::{header, Client, Response, Url};
use std::{collections::HashMap, str::FromStr};
use url::ParseError;

// 定义 HTTPie 的 CLI 的主入口，它包含若干个子命令
// 下面 /// 的注释是文档，clap 会将其作为 CLI 的帮助
/// A naive httpie implementation with Rust, can you imagine how easy it is?
#[derive(Parser, Debug)]
#[clap(author = "ithawk", version = "0.1.0", about, long_about = None)]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

// 子命令分别对应不同的 HTTP 方法，目前只支持 get / post
#[derive(clap::Subcommand, Debug)]
enum SubCommand {
    Get(Get),
    Post(Post),
    // 我们暂且不支持其它 HTTP 方法
}

// get 子命令
/// feed get with an url and we will retrieve the response for you
#[derive(Parser, Debug)]
struct Get {
    /// HTTP 请求的 URL
    #[clap(parse(try_from_str = parse_url))]
    url: String,
}

fn parse_url(s: &str) -> Result<String, ParseError> {
    // 这里我们仅仅检查一下 URL 是否合法
    match s.parse::<Url>() {
        Ok(url) => Ok(s.into()),
        Err(e) => Err(e),
    }
}

// post 子命令。需要输入一个 URL，和若干个可选的 key=value，用于提供 json body
/// feed post with an url and optional key=value pairs. We will post the data
/// as JSON, and retrieve the response for you
#[derive(Parser, Debug)]
struct Post {
    /// HTTP 请求的 URL
    #[clap(parse(try_from_str = parse_url))]
    url: String,
    /// HTTP 请求的 body
    #[clap(parse(try_from_str = parse_kv_pair))]
    body: Vec<KvPair>,
}

/// 命令行中的 key=value 可以通过 parse_kv_pair 解析成 KvPair 结构
#[derive(Debug, PartialEq)]
struct KvPair {
    k: String,
    v: String,
}

/// 当我们实现 FromStr trait 后，可以用 str.parse() 方法将字符串解析成 KvPair
impl FromStr for KvPair {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 使用 = 进行 split，这会得到一个迭代器
        let mut split = s.split("=");
        let err = || anyhow!(format!("Failed to parse {}", s));
        Ok(Self {
            // 从迭代器中取第一个结果作为 key，迭代器返回 Some(T)/None
            // 我们将其转换成 Ok(T)/Err(E)，然后用 ? 处理错误
            k: (split.next().ok_or_else(err)?).to_string(),
            // 从迭代器中取第二个结果作为 value
            v: (split.next().ok_or_else(err)?).to_string(),
        })
    }
}

/// 因为我们为 KvPair 实现了 FromStr，这里可以直接 s.parse() 得到 KvPair
fn parse_kv_pair(s: &str) -> Result<KvPair, anyhow::Error> {
    match s.parse() {
        Ok(s) => Ok(s),
        Err(e) => Err(e),
    }
}

/// D:\workspace\language\rust\rust-learn\target\debug>httpCli.exe get http:12344.com
/// Opts { subcmd: Get(Get { url: "http:12344.com" }) }
/// D:\workspace\language\rust\rust-learn\target\debug>httpCli.exe post http:12344.com a=1 b=2
/// Opts { subcmd: Post(Post { url: "http:12344.com", body: [KvPair { k: "a", v: "1" }, KvPair { k: "b", v: "2" }] }) }
// fn main() {
//     let opts: Opts = Opts::parse();
//     println!("{:?}", opts);
// }

/// 处理 get 子命令
async fn get(client: Client, args: &Get) -> Result<(), Box<dyn std::error::Error>> {
    let resp = client.get(&args.url).send().await?;
    Ok(print_resp(resp).await?)
}

/// 处理 post 子命令
async fn post(client: Client, args: &Post) -> Result<(), Box<dyn std::error::Error>> {
    let mut body = HashMap::new();
    for pair in args.body.iter() {
        body.insert(&pair.k, &pair.v);
    }
    let resp = client.post(&args.url).json(&body).send().await?;
    Ok(print_resp(resp).await?)
}

// 打印服务器版本号 + 状态码
fn print_status(resp: &Response) {
    let status = format!("{:?} {}", resp.version(), resp.status()).blue();
    println!("{}\n", status);
}
// 打印服务器返回的 HTTP header
fn print_headers(resp: &Response) {
    for (name, value) in resp.headers() {
        println!("{}: {:?}", name.to_string().green(), value);
    }
    print!("\n");
}

/// 打印服务器返回的 HTTP body
fn print_body(m: Option<Mime>, body: &String) {
    match m {
        // 对于 "application/json" 我们 pretty print
        Some(v) if v == mime::APPLICATION_JSON => {
            println!("{}", jsonxf::pretty_print(body).unwrap().cyan())
        }
        // 其它 mime type，我们就直接输出
        _ => println!("{}", body),
    }
}

/// 打印整个响应
async fn print_resp(resp: Response) -> Result<(), Box<dyn std::error::Error>> {
    print_status(&resp);
    print_headers(&resp);
    let mime = get_content_type(&resp);
    let body = resp.text().await?;
    print_body(mime, &body);
    Ok(())
}

/// 将服务器返回的 content-type 解析成 Mime 类型
fn get_content_type(resp: &Response) -> Option<Mime> {
    resp.headers()
        .get(header::CONTENT_TYPE)
        .map(|v| v.to_str().unwrap().parse().unwrap())
}

/// 程序的入口函数，因为在 HTTP 请求时我们使用了异步处理，所以这里引入 tokio
/// D:\workspace\language\rust\rust-learn\target\debug>http_cli.exe get https://httpbin.org/get
/// D:\workspace\language\rust\rust-learn\target\debug>http_cli.exe post https://httpbin.org/post a=1 b=2
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts: Opts = Opts::parse();
    let mut headers = header::HeaderMap::new();
    // 为我们的 HTTP 客户端添加一些缺省的 HTTP 头
    headers.insert("X-POWERED-BY", "Rust".parse()?);
    headers.insert(header::USER_AGENT, "Rust Httpie".parse()?);
    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;
    let result = match opts.subcmd {
        SubCommand::Get(ref args) => get(client, args).await?,
        SubCommand::Post(ref args) => post(client, args).await?,
    };
    Ok(result)
}

///     cargo test  ////
/// 仅在 cargo test 时才编译
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_url_works() {
        assert!(parse_url("abc").is_err());
        assert!(parse_url("http://abc.xyz").is_ok());
        assert!(parse_url("https://httpbin.org/post").is_ok());
    }

    #[test]
    fn parse_kv_pair_works() {
        assert!(parse_kv_pair("a").is_err());
        assert_eq!(
            parse_kv_pair("a=1").unwrap(),
            KvPair {
                k: "a".into(),
                v: "1".into(),
            }
        );
        assert_eq!(
            parse_kv_pair("b=").unwrap(),
            KvPair {
                k: "b".into(),
                v: "".into(),
            }
        );
    }
}
