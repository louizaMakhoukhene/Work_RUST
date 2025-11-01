//*DEFINITION DE MES STRCUT ET ENUM */


#[derive(Debug)]

struct Song{
    titre: String,
    artist: String,
    duration: Option<u32>,
}

#[derive(Debug)]

enum  Genre{
    Rock,
    Pop,
    Jazz,
    Other(String),
}

#[derive(Debug)]

struct Playlist{
    name: String,
    genre: Genre,
    songs: Vec<Song>,
}

//*J'AJOUTE UN SONG A LA PLAYLIST */
fn add_song(playlist: &mut Playlist, song: Song){

    playlist.songs.push(song);
}


//* JE RETOURNE UN OPTION DE LA MOYENNE DE LA DURÉE DE TTE LES SONGS QUI ONT UNE DURÉÉ CONNU  */
fn average_duration(playlist: &Playlist) -> Option<f64>{

    let mut total_duration:u32 = 0;
    let mut compteur:u32= 0;
    
    for song in &playlist.songs{          //ON PEUT ITERER SUR LES CHANPS DE STRUCTURES QUI SONT SEULEMENT DES COLLECTIONS 

        if let Some(duree) = song.duration{  //QUAND TU VEUX TRAITER QUE LE CAS SOME() ON UTILISE IF LET SOME(..)
            total_duration += duree;
            compteur +=1;
        }
    }
    if compteur > 0 {
        Some(total_duration as f64 / compteur as f64)
    }
    else {
        None
    }
}

fn long_songs(playlist: &Playlist) -> Vec<&Song>{

    let mut new_songs= vec![];

    for song in &playlist.songs{

        if let Some(duree) = song.duration{
            if duree > 300 {
                new_songs.push(song);
            }
        }
    }
    new_songs

}
//*INTO_ITER() DEPLACE LE VECTEUR DONC ON PERDS LE VECTEUR ORIGINAL */
//*ITER() PARCOURT PAR REFERENCE */
//*MATCHES!(VALEUR, MOTIF)=BOOL */
//*matches!(song.duration, Some(duree) if duree> 300 )= RENVOIE TRUE SI SONG.DURATION CONTINET UNE DUREE TQ DUREE> 300 */

pub fn long_songs_bis(playlist: &Playlist) -> Vec<&Song>{

    playlist.songs.iter().filter(|song| matches!(song.duration, Some(duree) if duree> 300 )).collect()
}




















fn main() {
    
}
