use std::{io::{stdin, Write}, fs::{File, read_dir}, path::Path, ffi::OsStr};

fn main() {
    let mut p_name = String::new();
    println!("Bienvenue sur Maketool !");
    println!("Quel est le nom de votre programme ? ");
    stdin()
    .read_line(&mut p_name)
    .expect("Une erreur est survenue !");
    println!("Votre programme s'appel donc {p_name}");
    let mut make_file = File::create("Makefile").unwrap();
    make_file.write_all(format!("NAME={p_name}").as_bytes());
    make_file.write(format!("CC=gcc\n").as_bytes());
    make_file.write(format!("SRC=").as_bytes());

    for file in read_dir(".").unwrap() {
        let f = file.unwrap();
        if let Some(ex) = Path::new(f.path().as_os_str()).extension() {
            if ex == "c" {
                make_file.write(format!("{} \\\n", f.file_name().as_os_str().to_str().unwrap()).as_bytes());
            }
        }
    }
    make_file.write(format!("\nCFLAGS = -Wall -Wextra -Werror\n").as_bytes());
    make_file.write(format!("OBJ = $(SRC:.c=.o)\n").as_bytes());
    make_file.write(format!("\n$(NAME):\n").as_bytes());
    make_file.write(format!("\t$(CC) -c $(SRC) $(CFLAGS)\n").as_bytes());
    make_file.write(format!("clear:\n").as_bytes());
    make_file.write(format!("\t@rm -f $(OBJ)\n").as_bytes());
    make_file.write(format!("fclean: clean\n").as_bytes());
    make_file.write(format!("\t@rm -f $(NAME)\n").as_bytes());
    make_file.write(format!("re: fclean all\n").as_bytes());
    make_file.write(format!("all: $(NAME)\n").as_bytes());
    make_file.write(format!(".PHONY:\n").as_bytes());
    make_file.write(format!("\tall clear fclean re bonus\n").as_bytes());
}
