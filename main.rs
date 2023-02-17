fn main() {
  loops();
}

fn loops() {
  let mult: u8 = 5;
  let mut counter = 0;

  while counter < 10 {
    counter += 1;
    if counter == 5 { // pula a multiplicação do 5 por 5
      continue;
    }
    println!("{} * {} = {} -- primeiro loop (while)", counter, counter, mult * mult);
  }

  counter = 0;
  
  loop {
    counter += 1;
    println!("{} * {} = {} -- segundo loop (loop)", counter, mult, counter * mult); 
    if counter == 5 {
      break;
    }
  }

  for i in 1..11 {
    println!("{} * {} = {} -- terceiro loop (for)", i, mult, i * mult);
  }

  let lang: &str = "PHP";
  
  let propourses: &str = match lang {
    "PHP" => "Web",
    "Kotlin" => "Android",
    "Python" => "Data Science",
    _ => "não definido"
  };

  println!("{} é uma linguagem para {}", lang, propourses);
  
}