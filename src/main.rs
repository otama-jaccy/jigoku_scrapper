use std::collections::HashSet;
use std::io::{BufReader, BufRead};
use std::fs::{File, read};
use std::iter::FromIterator;

#[derive(Debug)]
struct Movie {
    id: i32,
    title: String,
    genres: Vec<String>
}

#[derive(Debug)]
struct Tag {
    movie_id: i32,
    user_id: i32,
    tag: String,
    timestamp: i32
}

#[derive(Debug)]
struct Rating {
    movie_id: i32,
    user_id: i32,
    rating: f32,
    timestamp: i32
}

#[derive(Debug)]
struct MovieLens {
    movie_id: i32,
    user_id: i32,
    rating: f32,
    timestamp: i32,
    title: String,
    genres: Vec<String>
}

fn load_csv<T>(file_path: &str, builder: fn(Vec<String>) -> T) -> Vec<T> {
    println!("loading {}...", file_path);

    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let mut ret: Vec<T> = Vec::new();

    for result in reader.lines() {
        let s: Vec<String> = result.unwrap().split("::").map(|e| e.to_string()).collect();
        ret.push(builder(s))
    }

    println!("finished");
    ret
}

fn load_movies() -> Vec<Movie> {
    let movie_builder = |s: Vec<String>| {
        let generes: Vec<String> = s[2].split("|").map(|e| e.to_string()).collect();
        Movie {
            id: s[0].parse().unwrap(),
            title: s[1].to_string(),
            genres: generes
        }
    };

    load_csv("./ml-10M100K/movies.dat", movie_builder)
}

fn load_tags() -> Vec<Tag> {
    let tag_builder = |s: Vec<String>| {
        Tag {
            user_id: s[0].parse().unwrap(),
            movie_id: s[1].parse().unwrap(),
            tag: s[2].to_string(),
            timestamp: s[3].parse().unwrap()
        }
    };
    
    load_csv("./ml-10M100K/tags.dat", tag_builder)
}

fn load_ratings() -> Vec<Rating> {
    let rating_builder = |s: Vec<String>| {
        Rating {
            user_id: s[0].parse().unwrap(),
            movie_id: s[1].parse().unwrap(),
            rating: s[2].parse().unwrap(),
            timestamp: s[3].parse().unwrap()
        }
    };

    load_csv("./ml-10M100K/ratings.dat", rating_builder)
}

fn main() {
    let movies = load_movies();
    let tags = load_tags();
    let ratings = load_ratings();

    let unique_user_ids = ratings.iter().map(|e| {
        e.user_id
    }).collect::<HashSet<i32>>();

    let mut valid_user_ids = Vec::from_iter(unique_user_ids)[0..1000].to_vec();
    valid_user_ids.sort();

    let filtered_ratings: Vec<&Rating> = ratings.iter().filter(|e| {
        valid_user_ids.binary_search(&e.user_id).is_ok()
    }).collect();

    let mut movie_lens: Vec<MovieLens> = filtered_ratings.iter().map(|e| {
        let movie = movies.iter().find(|m| m.id == e.movie_id).unwrap();
        MovieLens {
            movie_id: movie.id,
            user_id: e.user_id,
            rating: e.rating,
            timestamp: e.timestamp,
            title: movie.title.to_string(),
            genres: movie.genres.clone()
        }
    }).collect();

    movie_lens.sort_by(|a, b| a.user_id.partial_cmp(&b.user_id).unwrap());

    println!("{:?}", &movie_lens[0..10]);
}
