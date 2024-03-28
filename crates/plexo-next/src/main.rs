use async_openai::{
    types::{
        ChatCompletionRequestAssistantMessageArgs, ChatCompletionRequestSystemMessageArgs,
        ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs,
    },
    Client,
};
use dotenv::dotenv;
use git2::{Buf, Diff, DiffDelta, DiffHunk, DiffLine, DiffOptions, Repository};
use std::{error::Error, io::BufWriter};
use std::{io::Write, str};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let client = Client::new();

    let path = "./playground/self";
    let repo = Repository::open(path)?;

    let branches = repo.branches(None).unwrap();

    println!(
        "Branches: \n{}\n",
        branches
            .into_iter()
            .map(|b| b.unwrap().0.name().unwrap().unwrap().to_string())
            .collect::<Vec<_>>()
            .join("\n")
    );

    // Diff::from_buffer(buffer)

    let diff = repo.diff_index_to_workdir(None, None)?;

    println!("deltas: \n{:?}\n", diff.deltas().collect::<Vec<_>>());

    println!("Diff: {:?}", diff.stats()?);

    let location = git2::ApplyLocation::Index;

    // let options = git2::ApplyOptions::new();

    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(512u16)
        .model("gpt-3.5-turbo")
        .messages([
            ChatCompletionRequestSystemMessageArgs::default()
                .content("You are a helpful assistant.")
                .build()?
                .into(),
            ChatCompletionRequestUserMessageArgs::default()
                .content("Who won the world series in 2020?")
                .build()?
                .into(),
            ChatCompletionRequestAssistantMessageArgs::default()
                .content("The Los Angeles Dodgers won the World Series in 2020.")
                .build()?
                .into(),
            ChatCompletionRequestUserMessageArgs::default()
                .content("Where was it played?")
                .build()?
                .into(),
        ])
        .build()?;

    // let mut diff_print_result = BufWriter::new(Vec::new());

    // diff.print(format, cb)

    // println!("diff: {}", str::from_utf8(diff_print_result.get_ref()).unwrap());

    let response = client.chat().create(request).await?;

    println!("response: {:?}", response.choices.first().unwrap().message.content);

    // let res = repo.apply(&diff, location, None);

    // println!("Apply result: {:?}", res);

    Ok(())
}

fn print_diff_line(_delta: DiffDelta, _hunk: Option<DiffHunk>, line: DiffLine, buff: &mut BufWriter<Vec<u8>>) -> bool {
    match line.origin() {
        '+' | '-' | ' ' => print!("{}", line.origin()),
        _ => {}
    }
    let l = str::from_utf8(line.content()).unwrap().to_string();

    buff.write_all(l.as_bytes()).unwrap();

    true
}
