use anyhow::{Context, Result};
use futures::future::try_join_all;
use reqwest::header;
use serde_json::Value;
use std::sync::Arc;
use tokio::task::JoinHandle;

//特定のリポジトリのスター数を返す非同期関数

async fn get_star_count(client: &reqwest::Client, repo: String ) -> Result<u64> {
    // get() は GitHub API に対する GET リクエストを作成
    // send() は作成した GET リクエストを送信し、レスポンスを受信して処理
    let resp: Value = client.get(&format!("https://api.github.com/repos/{}", repo)).send().await?.json().await?;
    // contextを利用しエラーメッセージにコンテキスト情報を追加→新しいエラーメッセージに
    let count = resp.get("stargazers_count").context("GitHub API error: stargazers_count is not found")?.as_u64().context("GitHub API error: stargazers_count is not an integer")?;
    Ok(count)
}

# [tokio::main]
async fn main() ->Result<()>{
    let mut headers = header::HeaderMap::new();
    headers.insert(
    header::ACCEPT,
        header::HeaderValue::from_static("application/vnd.github.v3+json")
    );
    let client = reqwest::Client::builder().user_agent("rust reqwest").default_headers(headers).build()?;
    // client は複数の非同期タスクで共有するため、std::sync::Arc で包んでいる
    let client = Arc::new(client);

    // 調べたいリポジトリのリスト
    // let repos = vec![
    //     "freeCodeCamp/freeCodeCamp".to_string(),
    //     "kamranahmedse/developer-roadmap".to_string(),
    // ];

    let repos = [
            "freeCodeCamp/freeCodeCamp".to_string(),
            "kamranahmedse/developer-roadmap".to_string(),
        ];

    let handles: Vec<JoinHandle<Result<u64>>> = repos.iter().map(|repo| {
        // tokip::spawnは'staticを要求するため、cloneする
        let client = client.clone();
        let repo = repo.clone();

        // clientとrepoをmoveする
        tokio::spawn(async move {
            //非同期タスクの実行結果はResult<u64>
            get_star_count(&client, repo).await
        })
    }).collect::<Vec<_>>();

    //repoに対応するstar数    // Vec<Result<T>> を Result<Vec<T>> に変換
    let stars: Vec<u64> = try_join_all(handles).await?.into_iter().collect::<Result<Vec<u64>>>()?;

    for (repo,star) in repos.iter().zip(stars){
        println!("{} has {} stars",repo ,star);
    }
    Ok(())
    }