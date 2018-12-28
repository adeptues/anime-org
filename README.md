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