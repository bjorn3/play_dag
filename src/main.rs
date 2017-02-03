mod play_dag;

use play_dag::{PlayDag, Song};

fn main(){
    let mut dag = PlayDag::new();
    
    let first = dag.add_song(Song { name: "First".to_string() });
    let then_1 = dag.add_song(Song { name: "Then 1".to_string() });
    let then_2 = dag.add_song(Song { name: "Then 2".to_string() });
    let last = dag.add_song(Song { name: "Last".to_string() });
    
    dag.link_song(first, then_1);
    dag.link_song(first, then_2);
    dag.link_song(then_1, last);
    dag.link_song(then_2, last);
    
    println!("{:#?}", dag);
    println!("{:?}", dag.get_next());
    println!("{:?}", dag.get_next());
    println!("{:?}", dag.get_next());
}