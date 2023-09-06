# ChatGPT English <> Japanese Conversion Inline

## User Stories
1. As a User, I can select one or more lines of English or Japanese text in a 
document and convert it, inline, to Japanese or English, respectively.
2. As a User, I should be able to specify the context and the degree of formality in the desired 
translation

## UI Notes
### UI Steps
1. Select the text
2. Copy the selected text
3. Enter a snippet shortcut, via TextExpander
4. Text is replaced with translated version

## Programming Notes
1. Use TextExpander to enter a keyboard shortcut
2. Uses [chatgpt_rs](https://docs.rs/chatgpt_rs/1.2.1/chatgpt/index.html) crate

## Todo
1. Add streaming reponses feature to begin printing the translation immediately
