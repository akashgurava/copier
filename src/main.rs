use anyhow::{anyhow, Error, Result};
use clap::Parser;
use s3::{creds::Credentials, Bucket, Region};

const REGION: Region = Region::EuWest1;

#[derive(Parser, Debug)]
#[clap(version)]
struct Args {
    /// Path to copy from.
    #[arg(short, long)]
    from: String,

    /// Path to copy to.
    #[arg(short, long)]
    to: String,

    #[arg(long, action)]
    verbose: bool,
}

fn path_to_bucket_key(path: &str) -> Result<(&str, &str), Error> {
    if !path.starts_with("s3://") {
        return Err(anyhow!("Path: `{}`. Invalid S3 path provided.", path));
    }

    let path = path.split("s3://").nth(1).unwrap();
    Ok(path.split_once("/").unwrap())
}

/// Lists your buckets.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let (from_bucket, from_key) = path_to_bucket_key(&args.from)?;
    let mut async_output_file = tokio::fs::File::create(&args.to)
        .await
        .expect("Unable to create file");

    if args.verbose {
        println!("Source bucket:      {}", &from_bucket);
        println!("Source key:         {}", &from_key);
        println!("Destination: {}", &args.to);
        println!();
    }
    let credentials = Credentials::default()?;
    let bucket = Bucket::new(from_bucket, REGION, credentials)?;
    let status_code = bucket
        .get_object_stream("/test.file", &mut async_output_file)
        .await?;

    dbg!(status_code);

    Ok(())
}
