use crate::{
    consts::PROGRAM_NAME,
    export::Export,
};

use crate::shortpaths::{expand_tilde, SP, sort_shortpaths};

use std::path::PathBuf;

/* NOTE: Consider generating an actual bash completions
   file instead of just generating the aliases script. */
pub struct BashExporter {
    shortpaths: Option<SP>,
}

pub fn fmt_bash_alias(name: &str, path: &PathBuf) -> String {
    format!("export {}=\"{}\"\n", name, path.display())
}

impl BashExporter {
    pub fn new(shortpaths: Option<SP>) -> BashExporter {
        BashExporter { shortpaths }
    }

    pub fn default() -> BashExporter {
        BashExporter::new(None)
    }
}

impl Export for BashExporter {
    /// Get the default local platform independent shell completions path 
    fn get_completions_path(&self) -> String {
        format!("completions/{}.bash", PROGRAM_NAME)
    }

    /// Get the system shell completions file path
    fn get_completions_sys_path(&self) -> String {
        format!("/usr/share/bash-completion/completions/{}", PROGRAM_NAME)
    }

    /** Let only users with equal permissions edit
      * the shell completions file */
    fn set_completions_fileperms(&self) -> String {
        todo!("Set user completion file perms");
        //String::from("")
    }

    fn gen_completions(&self) -> String {
        let mut output = String::from("#!/bin/bash\n\n");
        if let Some(shortpaths) = &self.shortpaths {
            let serialized: Vec<String> = shortpaths.iter().map(|(name, sp)| {
                let path = expand_tilde(sp.path()).unwrap();
                let shortpath = fmt_bash_alias(&name, &path);
                shortpath
            }).collect();

            //let mut output = String::from("#!/bin/bash\n\n");
            serialized.iter().for_each(|line| {
                output += line;
            });
            println!("output: {}", output);
        }
        output
    }

    fn set_shortpaths(&mut self, shortpaths: &SP) {
        self.shortpaths = Some(sort_shortpaths(shortpaths.to_owned()));
    }
}
