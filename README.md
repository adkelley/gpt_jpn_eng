# ChatGPT English <> Japanese Conversion

`gpt_jpn_eng` is an English <> Japanese language converter using the ChatGPT-rs wrapper 
over OpenAI's ChatGPT API in Rust. You can pair it with apps such as [TextExpander](https://textexpander.com/blog/what-is-textexpander)
to run a bash script (see `gpt_jpn_eng.sh`) that calls `gpt_jpn_eng` using the clipboard's contents 
as its argument. This pairing creates an inline translation experience without leaving your text document.

## Setup
Create a `.env` file with the following environment variables:
- OPENAI_API_KEY
- OPENAI_MAX_TOKENS

## Instructions (Typical Use Case)
1. While inside your document, select the text you wish to translate and copy it into the clipboard
2. Using TextExpander or Alfred, execute a bash script (see `gpt_jpn_eng.sh`) that calls `gpt_jpn_eng` 
using the clipboard's contents as its argument.  
3. The selected text is replaced with the translated version.

## What Could Possibly Go Wrong?
1. The number of tokens consumed during the translation will vary by the number of characters you wish
to translate.  If the resulting translation incomplete, set the
`OPENAPI_MAX_TOKENS` in your `.env` file to a higher number (> 1000).

2. Long text passages may take several seconds to translate and show up in your document.  I've 
improved the response of this program by adding streaming responses, which gradually builds the response message.
One caution, however. TextExpander appears to buffer stdout, waiting to print everything simultaneously
instead of continuously printing the translation. Worse is that the TextExpander script will time out on long
text passages. The workaround is to translate smaller text passages, or copy the entire selection and run it 
through Google Translate in your browser.

## User Stories
1. [x]  As a User, I can select one or more lines of English or Japanese text in a document and convert it, 
inline, to Japanese or English respectively.
2. [x]  As a User, I should see the translation begin printing almost immediately.

## Programming Notes
1. Uses [chatgpt_rs](https://docs.rs/chatgpt_rs/1.2.1/chatgpt/index.html) crate. The GitHub repository can be
found [here](https://github.com/Maxuss/chatgpt_rs)
