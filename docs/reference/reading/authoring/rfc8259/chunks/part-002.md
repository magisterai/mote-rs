# The JavaScript Object Notation (JSON) Data Interchange Format — part 2

Source: [The JavaScript Object Notation (JSON) Data Interchange Format](https://www.rfc-editor.org/rfc/rfc8259.txt) · RFC 8259.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.rfc-editor.org/rfc/rfc8259.txt).

Verbatim text slices in fixed-width blocks preserve grammar, examples, and diagrams.

[Previous](part-001.md) · [Next](part-003.md)

<a id="source-page-8-part-1"></a>

## Source page 8

```text

RFC 8259                          JSON                     December 2017


      int = zero / ( digit1-9 *DIGIT )

      minus = %x2D               ; -

      plus = %x2B                ; +

      zero = %x30                ; 0

   This specification allows implementations to set limits on the range
   and precision of numbers accepted.  Since software that implements
   IEEE 754 binary64 (double precision) numbers [IEEE754] is generally
   available and widely used, good interoperability can be achieved by
   implementations that expect no more precision or range than these
   provide, in the sense that implementations will approximate JSON
   numbers within the expected precision.  A JSON number such as 1E400
   or 3.141592653589793238462643383279 may indicate potential
   interoperability problems, since it suggests that the software that
   created it expects receiving software to have greater capabilities
   for numeric magnitude and precision than is widely available.

   Note that when such software is used, numbers that are integers and
   are in the range [-(2**53)+1, (2**53)-1] are interoperable in the
   sense that implementations will agree exactly on their numeric
   values.

7.  Strings

   The representation of strings is similar to conventions used in the C
   family of programming languages.  A string begins and ends with
   quotation marks.  All Unicode characters may be placed within the
   quotation marks, except for the characters that MUST be escaped:
   quotation mark, reverse solidus, and the control characters (U+0000
   through U+001F).

   Any character may be escaped.  If the character is in the Basic
   Multilingual Plane (U+0000 through U+FFFF), then it may be
   represented as a six-character sequence: a reverse solidus, followed
   by the lowercase letter u, followed by four hexadecimal digits that
   encode the character's code point.  The hexadecimal letters A through
   F can be uppercase or lowercase.  So, for example, a string
   containing only a single reverse solidus character may be represented
   as "\u005C".

   Alternatively, there are two-character sequence escape
   representations of some popular characters.  So, for example, a
   string containing only a single reverse solidus character may be
   represented more compactly as "\\".




Bray                         Standards Track                    [Page 8]
```

<a id="source-page-9-part-1"></a>

## Source page 9

```text

RFC 8259                          JSON                     December 2017


   To escape an extended character that is not in the Basic Multilingual
   Plane, the character is represented as a 12-character sequence,
   encoding the UTF-16 surrogate pair.  So, for example, a string
   containing only the G clef character (U+1D11E) may be represented as
   "\uD834\uDD1E".

      string = quotation-mark *char quotation-mark

      char = unescaped /
          escape (
              %x22 /          ; "    quotation mark  U+0022
              %x5C /          ; \    reverse solidus U+005C
              %x2F /          ; /    solidus         U+002F
              %x62 /          ; b    backspace       U+0008
              %x66 /          ; f    form feed       U+000C
              %x6E /          ; n    line feed       U+000A
              %x72 /          ; r    carriage return U+000D
              %x74 /          ; t    tab             U+0009
              %x75 4HEXDIG )  ; uXXXX                U+XXXX

      escape = %x5C              ; \

      quotation-mark = %x22      ; "

      unescaped = %x20-21 / %x23-5B / %x5D-10FFFF

8.  String and Character Issues

8.1.  Character Encoding

   JSON text exchanged between systems that are not part of a closed
   ecosystem MUST be encoded using UTF-8 [RFC3629].

   Previous specifications of JSON have not required the use of UTF-8
   when transmitting JSON text.  However, the vast majority of JSON-
   based software implementations have chosen to use the UTF-8 encoding,
   to the extent that it is the only encoding that achieves
   interoperability.

   Implementations MUST NOT add a byte order mark (U+FEFF) to the
   beginning of a networked-transmitted JSON text.  In the interests of
   interoperability, implementations that parse JSON texts MAY ignore
   the presence of a byte order mark rather than treating it as an
   error.







Bray                         Standards Track                    [Page 9]
```

<a id="source-page-10-part-1"></a>

## Source page 10

```text

RFC 8259                          JSON                     December 2017


8.2.  Unicode Characters

   When all the strings represented in a JSON text are composed entirely
   of Unicode characters [UNICODE] (however escaped), then that JSON
   text is interoperable in the sense that all software implementations
   that parse it will agree on the contents of names and of string
   values in objects and arrays.

   However, the ABNF in this specification allows member names and
   string values to contain bit sequences that cannot encode Unicode
   characters; for example, "\uDEAD" (a single unpaired UTF-16
   surrogate).  Instances of this have been observed, for example, when
   a library truncates a UTF-16 string without checking whether the
   truncation split a surrogate pair.  The behavior of software that
   receives JSON texts containing such values is unpredictable; for
   example, implementations might return different values for the length
   of a string value or even suffer fatal runtime exceptions.

8.3.  String Comparison

   Software implementations are typically required to test names of
   object members for equality.  Implementations that transform the
   textual representation into sequences of Unicode code units and then
   perform the comparison numerically, code unit by code unit, are
   interoperable in the sense that implementations will agree in all
   cases on equality or inequality of two strings.  For example,
   implementations that compare strings with escaped characters
   unconverted may incorrectly find that "a\\b" and "a\u005Cb" are not
   equal.

9.  Parsers

   A JSON parser transforms a JSON text into another representation.  A
   JSON parser MUST accept all texts that conform to the JSON grammar.
   A JSON parser MAY accept non-JSON forms or extensions.

   An implementation may set limits on the size of texts that it
   accepts.  An implementation may set limits on the maximum depth of
   nesting.  An implementation may set limits on the range and precision
   of numbers.  An implementation may set limits on the length and
   character contents of strings.

10.  Generators

   A JSON generator produces JSON text.  The resulting text MUST
   strictly conform to the JSON grammar.





Bray                         Standards Track                   [Page 10]
```

<a id="source-page-11-part-1"></a>

## Source page 11

```text

RFC 8259                          JSON                     December 2017


11.  IANA Considerations

   The media type for JSON text is application/json.

   Type name:  application

   Subtype name:  json

   Required parameters:  n/a

   Optional parameters:  n/a

   Encoding considerations:  binary

   Security considerations:  See RFC 8259, Section 12

   Interoperability considerations:  Described in RFC 8259

   Published specification:  RFC 8259

   Applications that use this media type:
      JSON has been used to exchange data between applications written
      in all of these programming languages: ActionScript, C, C#,
      Clojure, ColdFusion, Common Lisp, E, Erlang, Go, Java, JavaScript,
      Lua, Objective CAML, Perl, PHP, Python, Rebol, Ruby, Scala, and
      Scheme.

   Additional information:
      Magic number(s): n/a
      File extension(s): .json
      Macintosh file type code(s): TEXT

   Person & email address to contact for further information:
      IESG
      <iesg@ietf.org>

   Intended usage:  COMMON

   Restrictions on usage:  none

   Author:
      Douglas Crockford
      <douglas@crockford.com>

   Change controller:
      IESG
      <iesg@ietf.org>




Bray                         Standards Track                   [Page 11]
```

<a id="source-page-12-part-1"></a>

## Source page 12

```text

RFC 8259                          JSON                     December 2017


   Note:  No "charset" parameter is defined for this registration.
      Adding one really has no effect on compliant recipients.

12.  Security Considerations

   Generally, there are security issues with scripting languages.  JSON
   is a subset of JavaScript but excludes assignment and invocation.

   Since JSON's syntax is borrowed from JavaScript, it is possible to
   use that language's "eval()" function to parse most JSON texts (but
   not all; certain characters such as U+2028 LINE SEPARATOR and U+2029
   PARAGRAPH SEPARATOR are legal in JSON but not JavaScript).  This
   generally constitutes an unacceptable security risk, since the text
   could contain executable code along with data declarations.  The same
   consideration applies to the use of eval()-like functions in any
   other programming language in which JSON texts conform to that
   language's syntax.

13.  Examples

   This is a JSON object:

      {
        "Image": {
            "Width":  800,
            "Height": 600,
            "Title":  "View from 15th Floor",
            "Thumbnail": {
                "Url":    "http://www.example.com/image/481989943",
                "Height": 125,
                "Width":  100
            },
            "Animated" : false,
            "IDs": [116, 943, 234, 38793]
          }
      }

   Its Image member is an object whose Thumbnail member is an object and
   whose IDs member is an array of numbers.












Bray                         Standards Track                   [Page 12]
```

<a id="source-page-13-part-1"></a>

## Source page 13

```text

RFC 8259                          JSON                     December 2017


   This is a JSON array containing two objects:

      [
        {
           "precision": "zip",
           "Latitude":  37.7668,
           "Longitude": -122.3959,
           "Address":   "",
           "City":      "SAN FRANCISCO",
           "State":     "CA",
           "Zip":       "94107",
           "Country":   "US"
        },
        {
           "precision": "zip",
           "Latitude":  37.371991,
           "Longitude": -122.026020,
           "Address":   "",
           "City":      "SUNNYVALE",
           "State":     "CA",
           "Zip":       "94085",
           "Country":   "US"
        }
      ]

   Here are three small JSON texts containing only values:

   "Hello world!"

   42

   true



















Bray                         Standards Track                   [Page 13]
```

<a id="source-page-14-part-1"></a>

## Source page 14

```text

RFC 8259                          JSON                     December 2017


14.  References

14.1.  Normative References

   [ECMA-404] Ecma International, "The JSON Data Interchange Format",
              Standard ECMA-404,
              <http://www.ecma-international.org/publications/
              standards/Ecma-404.htm>.

   [IEEE754]  IEEE, "IEEE Standard for Floating-Point Arithmetic",
              IEEE 754.

   [RFC2119]  Bradner, S., "Key words for use in RFCs to Indicate
              Requirement Levels", BCP 14, RFC 2119,
              DOI 10.17487/RFC2119, March 1997,
              <https://www.rfc-editor.org/info/rfc2119>.

   [RFC3629]  Yergeau, F., "UTF-8, a transformation format of ISO
              10646", STD 63, RFC 3629, DOI 10.17487/RFC3629, November
              2003, <https://www.rfc-editor.org/info/rfc3629>.

   [RFC5234]  Crocker, D., Ed. and P. Overell, "Augmented BNF for Syntax
              Specifications: ABNF", STD 68, RFC 5234,
              DOI 10.17487/RFC5234, January 2008,
              <https://www.rfc-editor.org/info/rfc5234>.

   [RFC8174]  Leiba, B., "Ambiguity of Uppercase vs Lowercase in RFC
              2119 Key Words", BCP 14, RFC 8174, DOI 10.17487/RFC8174,
              May 2017, <https://www.rfc-editor.org/info/rfc8174>.

   [UNICODE]  The Unicode Consortium, "The Unicode Standard",
              <http://www.unicode.org/versions/latest/>.

14.2.  Informative References

   [ECMA-262] Ecma International, "ECMAScript Language Specification",
              Standard ECMA-262, Third Edition, December 1999,
              <http://www.ecma-international.org/publications/files/
              ECMA-ST-ARCH/
              ECMA-262,%203rd%20edition,%20December%201999.pdf>.

   [Err3607]  RFC Errata, Erratum ID 3607, RFC 4627,
              <https://www.rfc-editor.org/errata/eid3607>.

   [Err3915]  RFC Errata, Erratum ID 3915, RFC 7159,
              <https://www.rfc-editor.org/errata/eid3915>.





Bray                         Standards Track                   [Page 14]
```

<a id="source-page-15-part-1"></a>

## Source page 15

```text

RFC 8259                          JSON                     December 2017


   [Err4264]  RFC Errata, Erratum ID 4264, RFC 7159,
              <https://www.rfc-editor.org/errata/eid4264>.

   [Err4336]  RFC Errata, Erratum ID 4336, RFC 7159,
              <https://www.rfc-editor.org/errata/eid4336>.

   [Err4388]  RFC Errata, Erratum ID 4388, RFC 7159,
              <https://www.rfc-editor.org/errata/eid4388>.

   [Err607]   RFC Errata, Erratum ID 607, RFC 4627,
              <https://www.rfc-editor.org/errata/eid607>.

   [RFC4627]  Crockford, D., "The application/json Media Type for
              JavaScript Object Notation (JSON)", RFC 4627,
              DOI 10.17487/RFC4627, July 2006,
              <https://www.rfc-editor.org/info/rfc4627>.

   [RFC7159]  Bray, T., Ed., "The JavaScript Object Notation (JSON) Data
              Interchange Format", RFC 7159, DOI 10.17487/RFC7159, March
              2014, <https://www.rfc-editor.org/info/rfc7159>.































Bray                         Standards Track                   [Page 15]
```

