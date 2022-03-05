" Sytnax file for wal

if exists('b:current_syntax')
    finish
endif

syn match walFunction '\v-'
syn match walFunction '\v(-\>)|(\<-)'
syn match walFunction '\v\+'
syn match walFunction '\v\$'
syn match walFunction '\v\=\<'
syn match walFunction '\v\=\='
syn match walFunction '\v\@'
syn match walFunction '\v\@\>'
syn match walFunction '\v\%\$'

syn match walNumber '\d\+' display
syn region walString start='\'' end='\'' display
syn region walComment start='#' end='\n' display 

let b:current_syntax='wal'

hi def link walFunction Statement
hi def link walString String
hi def link walNumber Number
hi def link walComment Comment
