use std::{
    fs::{read_dir, File},
    io::{stdin, Write},
    path::Path,
};

struct Makefile {
    name: String,
    // files: Vec<String>,
    compiler: String,
    is_libs: bool,
    cmd_run: bool,
    // cmd_libs: bool,
    // libs: String,
}

impl Makefile {
    fn new(name: String) -> Self {
        Makefile {
            name,
            // files: Vec::new(),
            compiler: String::new(),
            is_libs: false,
            cmd_run: false,
            // cmd_libs: false,
            // libs: String::new(),
        }
    }
}

fn main() {
    /* -------------------------------------------------------------------------- */
    /*                                  Bienvenu                                  */
    /* -------------------------------------------------------------------------- */

    let mut name = String::new();
    println!("Bienvenue sur Maketool !");
    println!("Quel est le nom de votre programme ? ");
    stdin()
        .read_line(&mut name)
        .expect("La recuperation de l'entre utilisateur a echoue.");
    let mut makefile = Makefile::new(name);

    /* -------------------------------------------------------------------------- */
    /*                           Parametrage Du Makefile                          */
    /* -------------------------------------------------------------------------- */

    loop {
        println!("Quel type de Makefile souhaitez-vous generer ?");
        println!("1) Un programme avec un main");
        println!("2) Une bibliotheque");
        let mut cmd = String::new();
        stdin()
            .read_line(&mut cmd)
            .expect("La recuperation de l'entre utilisateur a echoue.");
        match cmd.as_str() {
            "1\n" => break,
            "2\n" => {
                makefile.is_libs = true;
                break;
            }
            _ => (),
        }
    }

    /* -------------------------------- Compiler -------------------------------- */
    println!("Quel compilateur souhaitez-vous uiliser :");
    stdin()
        .read_line(&mut makefile.compiler)
        .expect("La recuperation de l'entre utilisateur a echoue.");

    /* ----------------------------------- Run ---------------------------------- */
    loop {
        println!("Souhaitez-vous ajouer une commande 'run'[y/n]");
        let mut cmd = String::new();
        stdin()
            .read_line(&mut cmd)
            .expect("La recuperation de l'entre utilisateur a echoue.");
        match cmd.as_str() {
            "y\n" => {
                makefile.cmd_run = true;
                break;
            }
            "n\n" => break,
            _ => (),
        }
    }

    // /* ---------------------------------- Libs ---------------------------------- */
    // loop {
    //     println!("Souhaitez-vous inclure une bibliotheque a votre Makefile [y/n]");
    //     let mut cmd = String::new();
    //     stdin()
    //         .read_line(&mut cmd)
    //         .expect("La recuperation de l'entre utilisateur a echoue.");
    //     match cmd.as_str() {
    //         "y\n" => {
    //             makefile.cmd_libs = true;
    //             println!("Nom de votre bibliotheque: ");
    //             stdin()
    //                 .read_line(&mut makefile.libs)
    //                 .expect("La recuperation de l'entre utilisateur a echoue.");
    //             break;
    //         }
    //         "n\n" => break,
    //         _ => (),
    //     }
    // }

    /* -------------------------------------------------------------------------- */
    /*                            Creation du Makefile                            */
    /* -------------------------------------------------------------------------- */

    let mut file = match File::create("Makefile") {
        Ok(file) => file,
        Err(_) => panic!("Le Makefile n'a pas pu etre cree"),
    };

    /* ---------------------------------- NAME ---------------------------------- */
    file.write_all(format!("NAME={}", makefile.name).as_bytes())
        .expect("Le programme n'a pas pu ecrire dans le fichier");

    /* ----------------------------------- CC ----------------------------------- */
    file.write(format!("CC={}\n", makefile.compiler).as_bytes())
        .expect("Le programme n'a pas pu ecrire dans le fichier");

    /* ----------------------------------- SRC ---------------------------------- */
    file.write(format!("SRC=").as_bytes())
        .expect("Le programme n'a pas pu ecrire dans le fichier");
    for c_file in read_dir(".").expect("Le dossier n'existe pas") {
        let c_file = c_file.unwrap();
        if makefile.is_libs && c_file.file_name().as_os_str().to_str().unwrap() == "main.c"
        {
            continue;
        }
        if let Some(ext) = Path::new(c_file.path().as_os_str()).extension() {
            if ext == "c" {
                file.write(
                    format!("{} \\\n", c_file.file_name().as_os_str().to_str().unwrap()).as_bytes(),
                )
                .expect("Le programme n'a pas pu ecrire dans le fichier");
            }
        }
    }

    /* --------------------------------- CFLAGS --------------------------------- */
    file.write(format!("\nCFLAGS = -Wall -Wextra -Werror\n").as_bytes())
        .expect("Le programme n'a pas pu ecrire dans le fichier");

    /* ----------------------------------- OBJ ---------------------------------- */
    file.write(format!("OBJ = $(SRC:.c=.o)\n").as_bytes())
        .expect("Le programme n'a pas pu ecrire dans le fichier");

    /* -------------------------------------------------------------------------- */
    /*                                    RULES                                   */
    /* -------------------------------------------------------------------------- */

    /* --------------------------------- $(NAME) -------------------------------- */
    file.write(format!("\n$(NAME):\n").as_bytes())
        .expect("Le programme n'a pas pu ecrire dans le fichier");
    match makefile.is_libs {
        true => {
            file.write(format!("\t$(CC) -c $(SRC) $(CFLAGS)\n").as_bytes())
                .expect("Le programme n'a pas pu ecrire dans le fichier");
            file.write(format!("\tar rcs $(NAME) $(OBJ)\n").as_bytes())
                .expect("Le programme n'a pas pu ecrire dans le fichier")
        }
        false => file
            .write(format!("\t$(CC) $(SRC) $(CFLAGS)\n").as_bytes())
            .expect("Le programme n'a pas pu ecrire dans le fichier"),
    };

    /* ----------------------------------- RUN ---------------------------------- */
    if makefile.cmd_run {
        file.write(format!("run: $(NAME)\n").as_bytes())
            .expect("Le programme n'a pas pu ecrire dans le fichier");
        file.write(format!("\t./a.out\n").as_bytes())
            .expect("Le programme n'a pas pu ecrire dans le fichier");
    }

    /* ---------------------------------- CLEAN --------------------------------- */
    file.write(format!("clean:\n").as_bytes())
        .expect("Le programme n'a pas pu ecrire dans le fichier");
    file.write(format!("\trm -f $(OBJ)\n").as_bytes())
        .expect("Le programme n'a pas pu ecrire dans le fichier");

    /* --------------------------------- FCLEAN --------------------------------- */
    file.write(format!("fclean: clean\n").as_bytes())
        .expect("Le programme n'a pas pu ecrire dans le fichier");
    file.write(format!("\trm -f $(NAME)\n").as_bytes())
        .expect("Le programme n'a pas pu ecrire dans le fichier");

    /* ----------------------------------- RE ----------------------------------- */
    file.write(format!("re: fclean all\n").as_bytes())
        .expect("Le programme n'a pas pu ecrire dans le fichier");

    /* ----------------------------------- ALL ---------------------------------- */
    file.write(format!("all: $(NAME)\n").as_bytes())
        .expect("Le programme n'a pas pu ecrire dans le fichier");

    /* --------------------------------- .PHONY --------------------------------- */
    file.write(format!(".PHONY:\n").as_bytes())
        .expect("Le programme n'a pas pu ecrire dans le fichier");
    file.write(format!("\tall clear fclean re\n").as_bytes())
        .expect("Le programme n'a pas pu ecrire dans le fichier");

    println!("Generation du Makefile termine !");

}
