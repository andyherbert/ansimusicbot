# About ANSiMusic

### Tones
`A`-`G`  Plays A, B, ..., G in current octave. `#` or `+` after the note specifies sharp. `-` after the note specifies flat.

### Octaves
`O<n>`  Sets the current octave. There are seven octaves, numbered `0`-`6`.  
`>` increases octave by 1. Octaves cannot go beyond 6. `>>` increases by 2 octaves, as so on. `<`  decreases octave by 1. Octave cannot drop below 0. `<<` Decreases by 2 octaves, etc.

**Note:**  
`N<n>`  Plays note *n*. The range for *n* is `0`-`84`. In the seven possible octaves, there are 84 notes.  When *n* is `0`, this means a rest.

### Length
`L<n>`  Sets length of a note. The range for *n* is `1`-`64`.  `L 4` is a quarter note,  `L 1` is a whole note, etc.

The length may also follow the note when a change of length only is desired for a particular note. For example, `A 16` can be equivalent to `L 16 A`.

### Tempo
`T<n>`  Sets number of quarter notes per minute (`32`-`255`). The default for *n* is `120`.

### Staccato, Normal, & Legato
`MS`  Sets "Music Staccato" so that each note will play 3/4 of the time determined by the length `L`.  
`MN`  Sets "Music Normal" so that each note will play 7/8 of the time determined by the length `L`.  
`ML`  Sets "Music Legato" so that each note will play the full period set by length `L`.

### Pause
`P<n>`  Pause for the duration of *n* quarternotes. Specifies a pause, ranging from `1`-`64`. This option corresponds to the length of each note, set with `L<n>`.

### Dot
`.` A period after a note causes the note to play 3/2 times the length determined by `L` (length) times `T` (tempo). The period has the same meaning as in a musical score.  
Multiple periods can appear after a note. Each period adds a length equal to one half the length of the previous period. For example:  
`A.` plays 1 + 1/2 or 3/2 times the length  
`A..` plays 1 + 1/2 + 1/4 or 7/4 times the length  
Periods can appear after a `P` (pause). In this case, the pause length is scaled in the same way notes are scaled.

### Example:
    T80 L16 O4 ad<b-g>gd<bg>g-d<bg>gd<bg> gc<af>fc<af>ec<af>fc<af> f<b-ge>e<bge>e-<bge>e<bge>  e<afd>d<afd>d-<afd>d<afd> ad<b-g>gd<bg>g-d<bg>gd<bg> b-e-c<g->aec<g>a-ec<g>aec<g>> c<d<b-g>b-d<bg>ad<bg>b-d<bg> a<b-ge>g<bge>f<bge>e<bge
