use std::future::Future;
use std::io::Result;

pub fn stdinix<F>(mut cl: F) -> Result<()>
where
    F: FnMut(&str) -> Result<()>,
{
    let mut buf = String::new();
    while let Ok(true) = {
        buf.clear();
        std::io::stdin().read_line(&mut buf).map(|l| l > 0)
    } {
        cl(&buf)?;
    }

    Ok(())
}

pub async fn astdinix<F, Fut>(mut cl: F) -> Result<()>
where
    Fut: Future<Output = Result<()>>,
    F: FnMut(String) -> Fut,
{
    let mut buf = String::new();
    while let Ok(true) = {
        buf.clear();
        std::io::stdin().read_line(&mut buf).map(|l| l > 0)
    } {
        cl(buf.clone()).await?;
    }

    Ok(())
}
