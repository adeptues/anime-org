# anime organiser

This is a rust bin to organise individual episodes into folders named with the full english language name

- first scan entries and see if they match our initial regex for

```
[sub_group]_show_name_01.mkv
```

the following regex should work
```
^\[([a-zA-Z].*?)\]
```
this regex should extract the title of the show from the file

```
\](.*?)-
```

## Possible solutions

Create an ngram inverted index from our anidb dump of show so we can search by relevancy similar to 
https://github.com/BurntSushi/imdb-rename

or use a niave solution using regex and grep the anidb xml dump 

## Regex method

- first read all the entries from the filesystem see if they pass the first regex
- extract the showtext and place the files in a group by map using the show text as a map key
- then for each of the keys in the map search the anidb dump for a matching show name to get the aid entry
- get the official english title of the show
- update our show struct with the offical title
- then create an entry in the specifed folder if it does not already exist for the show title
- then move the individual files from their orginal location to the new folder we created

## TODO
- Add the logging crate to write sensible logs that track what was moved where
- add searching anidb's title dump for better names or all english names as an option
- refactor the file_map building and loginc into a fully types struct
- remove the hardcoded paths, either add them to a settings file or let them be passed in via command args *DONE*
- add docopt for command line parsing *DONE*
- Change the structopt from a string type to Path
- delpoy a version to the NAS to run every 5 mins or triggered via inotify on a directory

- Maybe as a seperate task project or an extra feature see if we can extract the watch history from kodi so we know what eps we've watched