use crate::db::*;
use base64::*;
use bytes::{Buf, Bytes};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

#[allow(clippy::type_complexity)]
pub fn import_rtst_vpk(
    db: &mut DatabaseSync,
    source_path: PathBuf,
) -> Result<(String, HashMap<String, Vec<Vec<u8>>>), sqlx::Error> {
    let author: String;
    let name: String;
    let description: String;

    let mut paths_to_keys = HashMap::new();
    let mut keys_to_sounds = HashMap::new();

    let mut pakfile = File::open(source_path.clone()).unwrap();
    let mut data_vec = Vec::new();
    pakfile.read_to_end(&mut data_vec).unwrap();
    let mut data = Bytes::from(data_vec);

    //Known rtst_vpk file headers take approximately 330 bytes.
    if data.len() > 330 {
        let first_bytes = data.split_to(7);
        let second_bytes = data.split_to(94);

        //Compare headers to ensure we are looking at an actual rtst_vpk file
        if b"\x02\x01\x04Root".eq(&first_bytes[..]) &&
            b"\x01\\RecursionTracker.PAKFormat, RTLibrary, Version=1.0.0.0, Culture=neutral, PublicKeyToken=null".eq(&second_bytes[..])
        {
            skip_next_item_short(&mut data); //1
            skip_next_item_short(&mut data); //objects
            skip_next_item(&mut data); //System.Object
            skip_next_item_short(&mut data); //6
            skip_next_item_short(&mut data); //Capacity
            skip_next_item_short(&mut data); //4
            data.advance(6); //Unknown function / format
            skip_next_item(&mut data); //ComponentInformation
            data.advance(2); //Unknown function / format

            skip_next_item(&mut data); //name header
            let pack_name_64 = get_next_item(&mut data);
            let name_vec = decode(pack_name_64).unwrap();
            name = String::from_utf8(name_vec).unwrap();
            println!("Pack Name: {name}");

            skip_next_item(&mut data); //Description header
            let description_64 = get_next_item(&mut data);
            let description_vec = decode(description_64).unwrap();
            description = String::from_utf8(description_vec).unwrap();
            println!("Pack Description: {description}");

            skip_next_item(&mut data); //Author header
            let author_64 = get_next_item(&mut data);
            let author_vec= decode(author_64).unwrap();
            author = String::from_utf8(author_vec).unwrap();
            println!("Pack Author: {author}");

            skip_next_item(&mut data); //sample image header
            data.advance(1); //0
            skip_next_item(&mut data); //classType header
            skip_next_item(&mut data); //AchievementOptionsComponents
            skip_next_item(&mut data); //Assembly header
            skip_next_item(&mut data); //RTPluginPS2
            data.advance(1); //unknown function / format
            skip_next_item(&mut data); //most of +GroupManager
            data.advance(1); //last 'l' from previous item (extra byte in this item compared to it's length field!)
            data.advance(2); //unknown function / format
            skip_next_item(&mut data); //achievementList header
            skip_next_item(&mut data); //System.string
            skip_next_item_short(&mut data); //AchiveementOptions

            let mut num_pairings_byte = get_next_item_short(&mut data);
            let num_pairings = num_pairings_byte.get_u8();
            for y in 0..num_pairings {
                println!("{}:", y +1);
                data.advance(2); //6,0 - unknown significance
                let achievement_key_64 = get_next_item(&mut data);
                //println!("raw >{achievement_key_64:?}<");
                let achievement_key_vec = decode(achievement_key_64).unwrap();
                let achievement_key = String::from_utf8(achievement_key_vec).unwrap();
                println!("key: {achievement_key}");

                data.advance(5); //2,0,0,1,6 - unknown significance
                skip_next_item(&mut data); //fileSoundPath header
                let file_sound_path_64 = get_next_item(&mut data);
                let file_sound_path_vec = decode(file_sound_path_64).unwrap();
                let file_sound_path = String::from_utf8(file_sound_path_vec).unwrap();
                println!("file_sound_path:{file_sound_path}");

                skip_next_item(&mut data); //soundEnabled header
                skip_next_item_short(&mut data); //boolean
                skip_next_item(&mut data); //imageEnabledInGame header
                skip_next_item_short(&mut data); //boolean
                skip_next_item(&mut data); //imageEnabledStreaming header
                skip_next_item_short(&mut data); //boolean

                let has_single_path = data.get_u8() == 6; //x05 means no value for this header, 6 means something is present
                let mut pak_sound_path = None;
                skip_next_item_short(&mut data); //pakSoundPath header
                if has_single_path {
                    let pak_sound_path_64 = get_next_item(&mut data);
                    let pak_sound_path_vec = decode(pak_sound_path_64).unwrap();
                    let pak_sound_path_str = String::from_utf8(pak_sound_path_vec).unwrap();
                    println!("pak_sound_path:{pak_sound_path_str}");
                    pak_sound_path = Some(pak_sound_path_str);
                } else {
                    data.advance(1); //consume trailing 0x00 if there was no pakSoundPath string.
                }

                skip_next_item(&mut data); //dynamicSounds header
                skip_next_item_short(&mut data); //1

                let has_multi_sounds = data.get_u8() == 7; //x05 means no values under this header, x7 means multiple sound entries are present
                skip_next_item_short(&mut data); //sounds header
                if has_multi_sounds {
                    skip_next_item(&mut data); //BasicAchievementSound
                    let mut num_sounds_byte = get_next_item_short(&mut data);
                    let num_sounds = num_sounds_byte.get_u8();
                    for _ in 0..num_sounds {
                        data.advance(5); //2,0,0,1,2 -- unknown significance

                        skip_next_item(&mut data); //soundFile header
                        let sound_file_path_64 = get_next_item(&mut data);
                        let sound_file_path_vec = decode(sound_file_path_64).unwrap();
                        let sound_file_path_str = String::from_utf8(sound_file_path_vec).unwrap();
                        println!("sound_file_path:{sound_file_path_str}");

                        skip_next_item(&mut data); //pakSoundFile header
                        let sound_pak_path_64 = get_next_item(&mut data);
                        let sound_pak_path_vec = decode(sound_pak_path_64).unwrap();
                        let sound_pak_path_str = String::from_utf8(sound_pak_path_vec).unwrap();
                        println!("sound_pak_path:{sound_pak_path_str}");

                        let list = paths_to_keys.entry(sound_pak_path_str).or_insert_with(Vec::new);
                        list.push(achievement_key.clone());
                    }
                } else {
                    if let Some(pak_path) = pak_sound_path {
                        let list = paths_to_keys.entry(pak_path).or_insert_with(Vec::new);
                        list.push(achievement_key);
                    }
                    data.advance(1); //consume trailing 0x00 if there weren't multiple sound entries
                }
            }

            skip_next_item(&mut data); //guid
            data.advance(18); //010000000000000000
            let has_embedded_files = data.get_u8() == 2; //0x02 when files present, 0x05 when none appended
            skip_next_item_short(&mut data); //componentInfo header
            if has_embedded_files {
                data.advance(3); //0,1,6
                skip_next_item(&mut data); //name Header
                skip_next_item(&mut data); //name in base64 again
                skip_next_item(&mut data); //description Header
                skip_next_item(&mut data); //description in base64 again
                skip_next_item(&mut data); //author Header
                skip_next_item(&mut data); //author in base64 again
                skip_next_item(&mut data); //sampleImage header
                data.advance(1); //0
                skip_next_item(&mut data); //classtype header
                skip_next_item(&mut data); //classtype base64 again
                skip_next_item(&mut data); //assembly header
                skip_next_item(&mut data); //assembly base64 again
                skip_next_item(&mut data); //backgroundImage header
                skip_next_item(&mut data); //background image name in base64
                skip_next_item(&mut data); //pakBackgroundImage header
                data.advance(1); //0
                skip_next_item(&mut data); //backgroundImageDisable header
                skip_next_item(&mut data); //boolean
                //XmlDictionary header - once again the entry size encoding is either incorrect or
                //inscrutable. All the files I've examined so far have the same content here, so we
                //can skip past it with a fixed length jump.
                data.advance(296);
                skip_next_item_short(&mut data); //System.string
                skip_next_item_short(&mut data); //ComponentData

                let mut num_embedded_byte= get_next_item_short(&mut data);
                let num_embedded = num_embedded_byte.get_u8();
                let mut keys_to_sounds_names = HashMap::new();
                for y in 0..num_embedded {
                    println!("{y}:");
                    data.advance(2); //6,0
                    let pak_path_64 = get_next_item(&mut data);
                    let pak_path_vec = decode(pak_path_64).unwrap();
                    let pak_path_str = String::from_utf8(pak_path_vec).unwrap();
                    println!("embedded file path:{pak_path_str}");
                    data.advance(5);//2,0,0,1,3 -- unknown purpose
                    skip_next_item(&mut data); //fileName header
                    let embed_file_name_64 = get_next_item(&mut data);
                    let embed_file_name_vec = decode(embed_file_name_64).unwrap();
                    let embed_file_name_str = String::from_utf8(embed_file_name_vec).unwrap();
                    println!("embedded file name:{embed_file_name_str}");
                    skip_next_item(&mut data); //data header

                    let mut file_size: usize = 0;
                    let size_field_length_byte = get_next_item_short(&mut data).get_u8();
                    if size_field_length_byte == 2 {
                        let short_len = data.get_u16_le();
                        file_size = short_len as usize;
                    } else if size_field_length_byte == 4 {
                        let int_len = data.get_u32_le();
                        file_size = int_len as usize;
                    } else if size_field_length_byte == 1 {
                        //assumed?
                        let byte_len = data.get_u8();
                        file_size = byte_len as usize;
                    }
                    println!("Embedded file byte length: {file_size}");

                    let sound_data = data.split_to(file_size);

                    skip_next_item(&mut data); //dataType header
                    skip_next_item_short(&mut data); //system.runtimetype
                    skip_next_item_short(&mut data); //system.byte


                    if let Some(key_list) = paths_to_keys.get(&pak_path_str) {
                        for key in key_list {
                            let sound_list = keys_to_sounds.entry(key.clone()).or_insert_with(Vec::<Vec::<u8>>::new);
                            let sound_vec: Vec<u8> = sound_data.clone().into_iter().collect();
                            sound_list.push(sound_vec.clone());

                            let sound_name_list = keys_to_sounds_names.entry(key.clone()).or_insert_with(Vec::<(Vec::<u8>, String)>::new);
                            sound_name_list.push((sound_vec, embed_file_name_str.clone()));
                        }
                    }
                }
                if let Some(filename_os_str) = source_path.file_name() {
                    let lossy_cow_str = filename_os_str.to_string_lossy();
                    db.store_voicepack_sync(name.clone(), lossy_cow_str.into_owned(), author, description, keys_to_sounds_names);
                } else {
                    db.store_voicepack_sync(name.clone(), "shenanigans.wut".to_owned(), author, description, keys_to_sounds_names);
                }

                Ok((name, keys_to_sounds))
            } else {
                println!("{} contains no audio files.",  source_path.display());
                Ok(("none".to_string(), HashMap::new()))
            }

        } else {
            println!("{} is not a recognized voicepack file.",  source_path.display());
            Ok(("none".to_string(), HashMap::new()))
        }
    } else {
        Ok(("none".to_string(), HashMap::new()))
    }
}

fn get_next_item(data: &mut Bytes) -> Bytes {
    data.advance(1);
    get_next_item_short(data)
}

fn get_next_item_short(data: &mut Bytes) -> Bytes {
    data.advance(1);
    let size_byte = data.get_u8();
    data.split_to(size_byte as usize)
}

fn skip_next_item(data: &mut Bytes) {
    data.advance(1);
    skip_next_item_short(data);
}

fn skip_next_item_short(data: &mut Bytes) {
    data.advance(1);
    let size_byte = data.get_u8();

    data.advance(size_byte as usize)
    /*let skipped = data.split_to(size_byte as usize);
    println!("SKipping: {skipped:?}");*/
}
