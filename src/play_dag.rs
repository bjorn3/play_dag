extern crate rand;

use std::fmt;
use std::collections::{HashMap, HashSet};
use self::rand::Rng;

#[derive(Clone)]
pub struct Song{
    pub name: String
}

impl fmt::Debug for Song{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "song{:?}", self.name)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub struct SongId(u32);

impl fmt::Debug for SongId{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "song#{}", self.0)
    }
}

#[derive(Debug)]
pub struct PlayDag{
    next_id: SongId,
    songs: HashMap<SongId, Song>,
    links: HashMap<SongId, HashSet<SongId>>,
    shuffler: Shuffler
}

impl PlayDag{
    pub fn new() -> PlayDag{
        PlayDag{
            next_id: SongId(0),
            songs: HashMap::new(),
            links: HashMap::new(),
            shuffler: Shuffler::None
        }
    }
    
    pub fn add_song(&mut self, song: Song) -> SongId{
        let id = self.next_id;
        self.songs.insert(id, song);
        self.links.insert(id, HashSet::new());
        self.next_id.0 += 1;
        id
    }
    
    pub fn link_song(&mut self, from: SongId, to: SongId){
        self.links.get_mut(&from).unwrap().insert(to);
    }
    
    pub fn get_next(&mut self) -> Option<Song>{
        match self.shuffler{
            Shuffler::None => {
                if let Some(next_song) = self.get(SongId(0)){
                    self.shuffler = Shuffler::RandomChild(SongId(0));
                    Some(next_song)
                }else{
                    None
                }
            },
            Shuffler::RandomChild(last_song) => {
                let mut rng = rand::thread_rng();
                let children = self.links.get(&last_song).unwrap();
                let count = children.iter().count();
                if count == 0 {
                    return None
                }
                let id = rng.gen::<usize>() % count;
                let next_song = children.iter().nth(id).unwrap();
                self.shuffler = Shuffler::RandomChild(*next_song);
                Some(self.get(next_song).unwrap())
            }
        }
    }
    
    fn get<S: ::std::borrow::Borrow<SongId>>(&self, song: S) -> Option<Song>{
        self.songs.get(song.borrow()).map(Clone::clone)
    }
}

#[derive(Debug)]
enum Shuffler{
    None,
    RandomChild(SongId)
}
