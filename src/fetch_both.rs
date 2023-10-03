use crate::libc_deb;

use colored::Colorize;
use snafu::ResultExt;
use snafu::Snafu;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("libc deb error: {}", source))]
    Deb { source: libc_deb::Error },

    #[snafu(display("failed writing to linker file: {}", source))]
    Write { source: std::io::Error },
}

pub type Result = std::result::Result<(), Error>;

/// Download linker compatible with libc version `ver` and save to directory
/// `dir`
pub fn fetch_libc_and_ld(ver: &String) -> Result {
  println!("{}{}", "fetching libc and linker version: ".green().bold(), ver);

  let string_short = ver.split('-').next().unwrap();
  let deb_file_name = format!("libc6_{}.deb", ver);
  let ld_name = "ld-linux-x86-64.so.2";
  let libc_name = "libc.so.6";
  let ld_out_name = format!("ld-{}.so", string_short);
  let libc_out_name = format!("libc-{}.so", string_short);


  libc_deb::write_ubuntu_pkg_file(&deb_file_name, &ld_name, ld_out_name).context(DebSnafu)?;
  libc_deb::write_ubuntu_pkg_file(&deb_file_name, &libc_name, libc_out_name).context(DebSnafu)?;
  Ok(())
}
