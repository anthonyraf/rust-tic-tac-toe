use std::io::{self, Write};

#[derive(PartialEq, Eq)]
struct Player {
    name: String,
    symbol: char,
}

#[derive(Debug)]
struct Coordinate {
    x: usize,
    y: usize,
}

fn main() {
    const EMPTY: char = ' ';
    let mut table: [[char; 3]; 3] = [[EMPTY; 3]; 3];

    let player_1: Player = Player {
        name: "Player 1".to_string(),
        symbol: 'x',
    };

    let player_2: Player = Player {
        name: "Player 2".to_string(),
        symbol: 'o',
    };

    let mut turn = &player_1;
    let mut turn_count: u8 = 0;

    let mut coor: Coordinate;
    let mut board: String;
    /*
    TODO: 
        * Error handling (index out of bounds: when x or y > 2)
        * Error handling (ParseIntError: conversion user input to int after splitting)
        * Feature: Ask for players name at start
    */
    loop {
        
        coor = ask_for_coor(&turn);
        
        while !is_valid_move(&table, &coor) {
            println!("\nVeuillez choisir un emplacement vide !!");
            coor = ask_for_coor(&turn);
        }

        play(&mut table, &turn, coor);
        turn_count += 1;

        board = get_board(&table);
        println!("{}\n", board);

        if game_is_over(&table, &turn) && turn_count < 9 {
            println!(
                "\nFélicitations {} [{}], vous avez gagné !\n",
                &turn.name, &turn.symbol
            );
            break;
        } else if turn_count == 9 {
            println!("\nEgalité entre les deux joueurs !!!\n");
            break;
        }

        if turn == &player_1 {
            turn = &player_2
        } else {
            turn = &player_1
        }
    }
}

fn ask_for_coor(player: &Player) -> Coordinate {
    let mut response: String = String::new();

    print!(
        "Entrez x,y (séparé par une ',') {}[{}] : ",
        player.name, player.symbol
    );
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut response)
        .expect("Echec de la lecture de l'entrée utilisateur!");

    let pos: Vec<usize> = response
        .trim()
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect();
    Coordinate {
        x: pos[0],
        y: pos[1],
    }
}

fn play(table: &mut [[char; 3]; 3], player: &Player, pos: Coordinate) {
    table[pos.x][pos.y] = player.symbol;
}

fn is_valid_move(table: &[[char;3];3], coor: &Coordinate) -> bool {
    table[coor.x][coor.y] == ' '
} 

fn get_board(table: &[[char; 3]; 3]) -> String {
    let mut board: String = String::new();
    let line: String = "+---".repeat(3) + "+\n";
    board += &line;
    for row in table {
        for col in row {
            board += "| ";
            board.push(*col);
            board += " ";
        }
        board += "|\n";
        board += &line;
    }
    board
}

fn game_is_over(table: &[[char; 3]; 3], player: &Player) -> bool {
    let mut is_over: bool = false;
    let mut horizontal: u8;
    let mut vertical: u8;
    let mut left_diag: u8 = 0;
    let mut right_diag: u8 = 0;

    for row in 0..table.len() {
        horizontal = 0;
        vertical = 0;
        for col in 0..table.len() {
            if table[row][col] == player.symbol {
                horizontal += 1
            }

            if table[col][row] == player.symbol {
                vertical += 1
            }
        }

        if table[row][row] == player.symbol {
            left_diag += 1;
        }
        if table[row][table.len() - row - 1] == player.symbol {
            right_diag += 1;
        }

        if horizontal == 3 || vertical == 3 || left_diag == 3 || right_diag == 3 {
            is_over = true;
            break;
        }
    }

    is_over
}
