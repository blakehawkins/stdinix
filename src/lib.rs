use std::future::Future;

pub fn stdinix<F>(mut cl: F) -> eyre::Result<()>
where
    F: FnMut(&str) -> eyre::Result<()>,
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

pub async fn astdinix<F, Fut>(mut cl: F) -> eyre::Result<()>
where
    Fut: Future<Output = eyre::Result<()>>,
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
