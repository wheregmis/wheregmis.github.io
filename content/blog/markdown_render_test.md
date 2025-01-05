+++
title = "Markdown Render Test"
date = "2024-03-15"
description = ""
image = "http://rustacean.net/assets/rustacean-flat-happy.png"
tags = ["rust", "web-dev", "tutorial"]
+++

# Markdown Feature Checklist

- [x] **Headings (h1-h6)**
- [x] **Links with hover effect**
- [x] **Code blocks with copy button**
- [x] **Ordered and unordered lists (nested)**
- [x] **Task lists (checkboxes)**
- [x] **Blockquotes**
- [ ] **Tables with alternating row colors**
- [x] **Horizontal rules**
- [!] **Text formatting (bold, italic, strikethrough, highlighted)** strikethrough and highlighted is not working
- [x] **Definition lists**
- [ ] **Abbreviations with hover effect**
- [x] **Keyboard shortcuts**
- [x] **Subscript and superscript**
- [x] **Details/Summary elements**
- [ ] **Footnotes**

---

## 1. Headings Example

# Heading 1
## Heading 2
### Heading 3
#### Heading 4
##### Heading 5
###### Heading 6

---

## 2. Links with Hover Effect

Hover over this link: [Example Link](# "This is a hover tooltip!")

---

## 3. Code Blocks with Copy Button

<pre>
<code class="language-js">
function helloWorld() {
  console.log("Hello, World!");
}
</code>
</pre>

*(Implementation of the copy button may depend on your Markdown renderer or additional scripts.)*

---

## 4. Ordered and Unordered Lists (Nested)

- Unordered item 1
  - Nested unordered item A
- Unordered item 2

1. Ordered item 1
   1. Nested ordered item 1-A
2. Ordered item 2

---

## 5. Task Lists (Checkboxes)

- [ ] Unchecked task
- [x] Checked task

---

## 6. Blockquotes

> This is a blockquote.
>
> > Nested blockquote.

---

## 7. Tables with Alternating Row Colors

| Column A | Column B | Column C |
|----------|----------|----------|
| Row 1A   | Row 1B   | Row 1C   |
| Row 2A   | Row 2B   | Row 2C   |
| Row 3A   | Row 3B   | Row 3C   |

*(Use CSS or your Markdown renderer’s settings for alternating row colors.)*

---

## 8. Horizontal Rules

---

Another rule:

***

---

## 9. Text Formatting

**Bold Text**  
*Italic Text*  
~~Strikethrough~~  

==Highlighted Text== (may depend on renderer)  

---

## 10. Definition Lists

Term 1
: Definition for Term 1

Term 2
: Definition for Term 2

---

## 11. Abbreviations with Hover Effect

Markdown allows abbreviations like `*[HTML]: HyperText Markup Language`

Example: We are using HTML in our project.

*[HTML]: HyperText Markup Language

---

## 12. Keyboard Shortcuts

Press <kbd>Ctrl</kbd> + <kbd>S</kbd> to save.

---

## 13. Subscript and Superscript

Subscript example: H<sub>2</sub>O  
Superscript example: 2<sup>nd</sup>

---

## 14. Details/Summary Elements

<details>
<summary>Click to expand</summary>
Inside details content
</details>

---

## 15. Footnotes

Here is a sentence with a footnote.[^1]

[^1]: This is the footnote content.
