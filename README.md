# Interactive PDF Analysis

_Interactive PDF Analysis_ (also called IPA) allows any researcher to explore the inner details of any PDF file. PDF files may be used to carry malicious payloads that exploit vulnerabilities, and issues of PDF viewer, or may be used in phishing campaigns as social engineering artefacts.
The goal of this software is to let any analyst go deep on its own the PDF file. Via IPA, you may extract important payload from PDF files, understand the relationship across objects, and infer elements that may be helpful for triage of malicious or untrusted payloads.

The main inspiration goes to the fantastic people behind Zynamics, and their excellent product, called PDF dissector.



## Simplifying analysis of PDF files

When I started reverse engineering malware, the main tool available for analysing malicious payloads consisted of [Didier Stevens](https://didierstevens.com/)'s excellent tools. Having become a _de facto_ standard, one of the main problems with these tools was the fact that they could be used from the command line, having to remember a very large combination of flags, reporting the numbers of the various objects. Although analysis and developers have to contend with all kinds of command-line tools on a daily basis, this does not mean that we cannot create a new graphical file inspection tool.

In fact, part of static analysis and reverse engineering fields also focuse on how to display the most salient information to the analyst from the point of view of user experience. Didier Stevens' tools, as well as peepdf, are already used and well broken in. However, the analyst could use something graphical in order to be able to understand the relationship between the various objects, to understand which pages they refer to and which object types (images, fonts, colours, metadata), to export stream content in a simple way and to see the content of dictionaries in table form.

The main source of inspiration comes from the tool developed by Zynamics called PDF-dissector: the excellent feedback from some former users and the constant requests to release it open source spurred me to spend a few days creating this tool.

## Features

* Extract and analyze metadata to identify the creator, creation date, modification history, and other essential details about the PDF file.
* Examine the structure of the PDF document by analyzing its objects (such as text, images, and fonts) and pages to understand their relationships, content, and layout.
* Visualize References that point to other objects or locations within the file, such as images, fonts, or specific sections. 
* Extract and save raw data streams from the PDF file to a specified location, allowing for detailed examination and analysis of the underlying binary content.
* Implement a lighter analysis that attempts to salvage usable information from a corrupted or partially damaged PDF file, even when traditional parsing methods fail.
* Does not require any additional software, libraries, or external services to function thanks to pdf-rs and Rust compatibility.

## Installation

The tool can be compiled with Rust and cargo. No dependencies are required apart from Rust, and the crates pulled.

```bash
git clone https://github.com/seekbytes/IPA
cd IPA

## compile with debug symbols, the binary is inside ./target/debug/IPA
cargo b 

## compile in release mode, the binary is inside ./target/release IPA
cargo b --release
```

## Credits
* [pdf-rs](https://github.com/pdf-rs/pdf) team for their awesome library (if you have any issues parsing a PDF file, you might want to open an issue for them)
* Zynamics for the inspiration of pdf dissector
* [egui](https://github.com/emilk/egui) for a solid immediate GUI mode

## Limitations

IPA has the following limitations for now:
- few heuristics (if you want to suggest some, please open a new issue or write me an email)
- no support for encrypted PDF (panics at opening, soon I'll implement a way to handle it if you know the password)
- not every PDF file is supported due to some strict requirements that pdf-rs assumes while opening it. If you have a PDF file that should be parsable, please open an issue to [pdf-rs](https://github.com/pdf-rs/pdf/issues) repository.
- some object types are not viewable natively. This is something I'm still brainstorming on: e.g. graphical elements, colors.

At the end, the tool can be really improved and I pretty bet some people will notice how bad my code is. If you're one of these, please let me know about potential improvements, better patterns, and any suggestion.

## Contact

If you have any issues, improvements, or you just want to text me, email [seekbytes@protonmail.com](mailto:seekbytes@protonmail.com).