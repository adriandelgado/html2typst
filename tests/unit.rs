use html2typst::parse_html;

#[test]
fn test_simple_p() {
    let typ = parse_html("<p>A paragraph</p>");
    assert_eq!(typ, "A paragraph");
}

#[test]
fn test_several_paragraphs() {
    let typ = parse_html("<p>asset afectado 1</p><p>asset afectado 2<br>asset afectado 3</p>");
    assert_eq!(
        typ,
        r#"asset afectado 1

asset afectado 2\
asset afectado 3"#
    );
}

#[test]
fn test_anchor() {
    let typ = parse_html(r#"<p><a href="https://youtu.be/rickroll">Interesting video</a></p>"#);
    assert_eq!(
        typ,
        r#"#link("https://youtu.be/rickroll")[Interesting video]"#
    );

    let typ = parse_html(
        r#"<p><a href="https://youtu.be/rickroll">Interesting video</a><a href="https://youtu.be/rickroll">Interesting video</a></p>"#,
    );
    assert_eq!(
        typ,
        r#"#link("https://youtu.be/rickroll")[Interesting video]#link("https://youtu.be/rickroll")[Interesting video]"#
    );
}

#[test]
fn test_image() {
    // TODO: handle figures
    let typ = parse_html(r#"<p><img src="asdf" alt="qwer"></p>"#);
    assert_eq!(
        typ,
        r#"#figure(caption: [qwer], image(alt: "qwer", "asdf"))"#
    );
}

#[test]
fn test_escaping() {
    // TODO: more scaping
    let typ = parse_html(
        r#"<p>*stars* asdf **double stars** qwer _single underscore_ zxcv __double underscore__</p>"#,
    );
    assert_eq!(
        typ,
        r#"\*stars\* asdf \*\*double stars\*\* qwer \_single underscore\_ zxcv \_\_double underscore\_\_"#
    );
}

#[test]
fn test_headers() {
    let typ = parse_html(r#"<h1 id="some-id">Title</h1><p>Some Text</p><h2>Subtitle</h2>"#);
    assert_eq!(
        typ,
        r#"= Title <some-id>
Some Text

== Subtitle"#
    );
}

#[test]
fn test_header_escaping() {
    let typ = parse_html(r#"<h1>= A header 2 + 2 = 4</h1>"#);
    assert_eq!(typ, r#"= \= A header 2 + 2 = 4"#);
}

#[test]
fn test_hyphen_escaping() {
    let typ = parse_html(r#"<p>- A pseudo-list</p>"#);
    assert_eq!(typ, r#"\- A pseudo-list"#);

    let typ = parse_html(r#"<p>    - remove starting whitespace</p>"#);
    assert_eq!(typ, r#"\- remove starting whitespace"#);
}

#[test]
fn test_entity_escaping() {
    let typ = parse_html(r#"&lt;img src="asdf qwer"&gt;"#);
    assert_eq!(typ, r#"\<img src="asdf qwer"\>"#);

    let typ = parse_html(r#"<img src="asdf &quot;zcxv&quot; qwer" alt="abc">"#);
    assert_eq!(
        typ,
        r#"#figure(caption: [abc], image(alt: "abc", "asdf \"zcxv\" qwer"))"#
    );
}

#[test]
fn test_blockquote() {
    let typ = parse_html(
        r#"<blockquote>Some saying
with deep meaning</blockquote>bla bla"#,
    );
    assert_eq!(
        typ,
        r#"#quote(block: true)[
Some saying
with deep meaning
]

bla bla"#
    );
}

#[test]
fn test_blockquote_nested() {
    let typ = parse_html(
        r#"<blockquote>A quote<blockquote>within a quote</blockquote>with some more text</blockquote>"#,
    );
    assert_eq!(
        typ,
        r#"#quote(block: true)[
A quote

#quote(block: true)[
within a quote
]

with some more text
]"#
    );
}

#[test]
#[ignore]
fn test_styles_with_spaces() {
    let md = parse_html(r#"It read:<s> Nobody will ever love you</s>"#);
    assert_eq!(md, r#"It read: #strike[Nobody will ever love you]"#)
}

#[test]
#[ignore]
fn test_styles_with_newlines() {
    let md = parse_html(
        r#"
And she said:<br/>
<s>We are all just prisoners here<br/>
Of our own device<br/>  </s>
And in the master's chambers<br/>
They gathered for the feast<br/>
<em>They stab it with their steely knives</em><br/>
<strong>But they just can't kill the beast<br/></strong>

"#,
    );
    assert_eq!(
        md,
        r#"And she said:\
#strike[We are all just prisoners here\
Of our own device\
] And in the master's chambers\
They gathered for the feast\
_They stab it with their steely knives_\
*But they just can't kill the beast\
*"#
    )
}

#[test]
#[ignore]
fn test_sub_sup() {
    let typ = parse_html(r#"A<sub>1</sub>B<sup>2</sup>"#);
    assert_eq!(typ, r#"A#sub[1]B#super[2]"#);
}

#[test]
fn test_lists() {
    let typ = parse_html(r#"<ul><li>One</li><li>Two</li><li>- Three</li></ul>"#);
    assert_eq!(
        typ,
        r#"- One
- Two
- \- Three"#
    );

    let typ = parse_html(r#"<ol><li>One</li><li>Two</li><li>+ Three</li></ol>"#);
    assert_eq!(
        typ,
        r#"+ One
+ Two
+ \+ Three"#
    );

    let typ = parse_html(r#"Numbers<ul><li>One</li><li>Two</li><li>Three</li></ul>"#);
    assert_eq!(
        typ,
        r#"Numbers

- One
- Two
- Three"#
    );
}

#[test]
#[ignore]
fn test_nested_tight_lists() {
    let typ = parse_html(
        r#"<ul><li>A<ol><li>X<ul><li>Deep</li></ul></li><li>Y</li><li>Z</li></ol></li><li>B</li><li>C</li>li</ul>"#,
    );
    assert_eq!(
        typ,
        r#"- A
  + X
    - Deep
  + Y
  + Z
- B
- C"#
    );
}

#[test]
#[ignore]
fn test_nested_not_tight_lists() {
    let typ = parse_html(
        r#"<ul><li>A<ol><li><p>X</p><ul><li><p>Deep</p></li></ul></li><li>Y</li><li>Z</li></ol></li><li>B<ul><li><p>No space</p></li></ul></li><li>C</li>li</ul>"#,
    );
    assert_eq!(
        typ,
        r#"- A
  + X

    - Deep

  + Y

  + Z
- B
  - No space
- C"#
    );
}

#[test]
#[ignore]
fn test_lists_br() {
    let typ = parse_html(r#"<ul><li>A<br/>X<br/>Y</li><li>B</li><li>C</li></ul>"#);
    assert_eq!(
        typ,
        r#"- A\
  X\
  Y
- B
- C"#
    );
}

#[test]
#[ignore]
fn test_lists_p() {
    let typ =
        parse_html(r#"<ul><li><p>X</p><p>Y</p></li><li>B<ol><li>Z</li></ol></li><li>C</li></ul>"#);
    assert_eq!(
        typ,
        r#"- X

  Y

- B
  + Z

- C"#
    );
}

#[test]
#[ignore]
fn test_table() {
    let typ = parse_html(
        r#"<table>
  <thead>
    <tr>
      <th scope="col">Header1</th>
      <th scope="col">Header2</th>
      <th scope="col">Header3</th>
      <th scope="col">Header4</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>col1</td>
      <td>col2</td>
      <td>col3</td>
      <td>col4</td>
    </tr>
  </tbody>
</table>"#,
    );
    assert_eq!(
        typ,
        r#"#figure(
  table(
    columns: 4,
    table.header([Header1], [Header2], [Header3], [Header4]),
    [col1], [col2], [col3], [col4],
  ),
)"#
    );
}

#[test]
#[ignore]
fn test_table_more_headers() {
    let typ = parse_html(
        r#"<table>
  <thead>
    <tr>
      <th scope="col">Header1</th>
      <th scope="col">Header2</th>
      <th scope="col">Header3</th>
      <th scope="col">Header4</th>
      <th scope="col">Header5</th>
      <th scope="col">Header6</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>col1</td>
      <td>col2</td>
      <td>col3</td>
      <td>col4</td>
    </tr>
  </tbody>
</table>"#,
    );
    assert_eq!(
        typ,
        r#"#figure(
  table(
    columns: 6,
    table.header([Header1], [Header2], [Header3], [Header4], [Header5], [Header6]),
    [col1], [col2], [col3], [col4], [], [],
  ),
)"#
    );
}

#[test]
#[ignore]
fn test_table_more_rows() {
    let typ = parse_html(
        r#"<table>
  <thead>
    <tr>
      <th scope="col">Header1</th>
      <th scope="col">Header2</th>
    </tr>
  </thead>
  <tbody>
    <tr>
      <td>col1</td>
      <td>col2</td>
      <td>col3</td>
      <td>col4</td>
    </tr>
  </tbody>
</table>"#,
    );
    assert_eq!(
        typ,
        r#"#figure(
  table(
    columns: 4,
    table.header([Header1], [Header2], [], []),
    [col1], [col2], [col3], [col4],
  ),
)"#
    );
}

#[test]
#[ignore]
fn test_rich_text() {
    let md = parse_html(
        r##"<p>finding description</p><p>new paragraph</p><p>a paragraph<br>with a linebreak</p><p>some rich text like <strong>bold</strong>, <em>italic</em>, <u>underlined</u>, <s>strike-through</s>, <mark data-color="#ffff25" style="background-color: #ffff25; color: #000000">highlighted</mark>, <strong>a <em>mix</em></strong><em> <u>of</u></em><u> them</u></p><ul><li><p>some</p><ol><li><p>first point</p></li><li><p>second point</p></li></ol></li><li><p>bullets</p><ul><li><p>even inside</p></li></ul></li></ul><p>some code here: <code>1 + 1 == 2</code></p><p>Here is a codeblock:</p><pre><code>1 + 1 == 2</code></pre><p></p>"##,
    );
    assert_eq!(
        md,
        "finding description\n\nnew paragraph\n\na paragraph\\\nwith a linebreak\n\nsome rich text like *bold*, _italic_, #underline[underlined], #strike[strike-through], #highlight[highlighted], *a _mix_*_ #underline[of]_#underline[ them]\n\n- some\n\n  + first point\n\n  + second point\n\n- bullets\n\n  - even inside\n\nsome code here: `1 + 1 == 2`\n\nHere is a codeblock:\n\n```\n1 + 1 == 2\n```"
    )
}
