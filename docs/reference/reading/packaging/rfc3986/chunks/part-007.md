# Uniform Resource Identifier (URI): Generic Syntax — part 7

Source: [Uniform Resource Identifier (URI): Generic Syntax](https://www.rfc-editor.org/rfc/rfc3986.txt) · RFC 3986.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.rfc-editor.org/rfc/rfc3986.txt).

Verbatim text slices in fixed-width blocks preserve grammar, examples, and diagrams.

[Previous](part-006.md) · [Next](part-008.md)

<a id="source-page-32-part-1"></a>

## Source page 32

```text

RFC 3986                   URI Generic Syntax               January 2005


      if defined(R.scheme) then
         T.scheme    = R.scheme;
         T.authority = R.authority;
         T.path      = remove_dot_segments(R.path);
         T.query     = R.query;
      else
         if defined(R.authority) then
            T.authority = R.authority;
            T.path      = remove_dot_segments(R.path);
            T.query     = R.query;
         else
            if (R.path == "") then
               T.path = Base.path;
               if defined(R.query) then
                  T.query = R.query;
               else
                  T.query = Base.query;
               endif;
            else
               if (R.path starts-with "/") then
                  T.path = remove_dot_segments(R.path);
               else
                  T.path = merge(Base.path, R.path);
                  T.path = remove_dot_segments(T.path);
               endif;
               T.query = R.query;
            endif;
            T.authority = Base.authority;
         endif;
         T.scheme = Base.scheme;
      endif;

      T.fragment = R.fragment;

5.2.3.  Merge Paths

   The pseudocode above refers to a "merge" routine for merging a
   relative-path reference with the path of the base URI.  This is
   accomplished as follows:

   o  If the base URI has a defined authority component and an empty
      path, then return a string consisting of "/" concatenated with the
      reference's path; otherwise,








Berners-Lee, et al.         Standards Track                    [Page 32]
```

<a id="source-page-33-part-1"></a>

## Source page 33

```text

RFC 3986                   URI Generic Syntax               January 2005


   o  return a string consisting of the reference's path component
      appended to all but the last segment of the base URI's path (i.e.,
      excluding any characters after the right-most "/" in the base URI
      path, or excluding the entire base URI path if it does not contain
      any "/" characters).

5.2.4.  Remove Dot Segments

   The pseudocode also refers to a "remove_dot_segments" routine for
   interpreting and removing the special "." and ".." complete path
   segments from a referenced path.  This is done after the path is
   extracted from a reference, whether or not the path was relative, in
   order to remove any invalid or extraneous dot-segments prior to
   forming the target URI.  Although there are many ways to accomplish
   this removal process, we describe a simple method using two string
   buffers.

   1.  The input buffer is initialized with the now-appended path
       components and the output buffer is initialized to the empty
       string.

   2.  While the input buffer is not empty, loop as follows:

       A.  If the input buffer begins with a prefix of "../" or "./",
           then remove that prefix from the input buffer; otherwise,

       B.  if the input buffer begins with a prefix of "/./" or "/.",
           where "." is a complete path segment, then replace that
           prefix with "/" in the input buffer; otherwise,

       C.  if the input buffer begins with a prefix of "/../" or "/..",
           where ".." is a complete path segment, then replace that
           prefix with "/" in the input buffer and remove the last
           segment and its preceding "/" (if any) from the output
           buffer; otherwise,

       D.  if the input buffer consists only of "." or "..", then remove
           that from the input buffer; otherwise,

       E.  move the first path segment in the input buffer to the end of
           the output buffer, including the initial "/" character (if
           any) and any subsequent characters up to, but not including,
           the next "/" character or the end of the input buffer.

   3.  Finally, the output buffer is returned as the result of
       remove_dot_segments.





Berners-Lee, et al.         Standards Track                    [Page 33]
```

<a id="source-page-34-part-1"></a>

## Source page 34

```text

RFC 3986                   URI Generic Syntax               January 2005


   Note that dot-segments are intended for use in URI references to
   express an identifier relative to the hierarchy of names in the base
   URI.  The remove_dot_segments algorithm respects that hierarchy by
   removing extra dot-segments rather than treat them as an error or
   leaving them to be misinterpreted by dereference implementations.

   The following illustrates how the above steps are applied for two
   examples of merged paths, showing the state of the two buffers after
   each step.

      STEP   OUTPUT BUFFER         INPUT BUFFER

       1 :                         /a/b/c/./../../g
       2E:   /a                    /b/c/./../../g
       2E:   /a/b                  /c/./../../g
       2E:   /a/b/c                /./../../g
       2B:   /a/b/c                /../../g
       2C:   /a/b                  /../g
       2C:   /a                    /g
       2E:   /a/g

      STEP   OUTPUT BUFFER         INPUT BUFFER

       1 :                         mid/content=5/../6
       2E:   mid                   /content=5/../6
       2E:   mid/content=5         /../6
       2C:   mid                   /6
       2E:   mid/6

   Some applications may find it more efficient to implement the
   remove_dot_segments algorithm by using two segment stacks rather than
   strings.

      Note: Beware that some older, erroneous implementations will fail
      to separate a reference's query component from its path component
      prior to merging the base and reference paths, resulting in an
      interoperability failure if the query component contains the
      strings "/../" or "/./".













Berners-Lee, et al.         Standards Track                    [Page 34]
```

<a id="source-page-35-part-1"></a>

## Source page 35

```text

RFC 3986                   URI Generic Syntax               January 2005


5.3.  Component Recomposition

   Parsed URI components can be recomposed to obtain the corresponding
   URI reference string.  Using pseudocode, this would be:

      result = ""

      if defined(scheme) then
         append scheme to result;
         append ":" to result;
      endif;

      if defined(authority) then
         append "//" to result;
         append authority to result;
      endif;

      append path to result;

      if defined(query) then
         append "?" to result;
         append query to result;
      endif;

      if defined(fragment) then
         append "#" to result;
         append fragment to result;
      endif;

      return result;

   Note that we are careful to preserve the distinction between a
   component that is undefined, meaning that its separator was not
   present in the reference, and a component that is empty, meaning that
   the separator was present and was immediately followed by the next
   component separator or the end of the reference.

5.4.  Reference Resolution Examples

   Within a representation with a well defined base URI of

      http://a/b/c/d;p?q

   a relative reference is transformed to its target URI as follows.







Berners-Lee, et al.         Standards Track                    [Page 35]
```

<a id="source-page-36-part-1"></a>

## Source page 36

```text

RFC 3986                   URI Generic Syntax               January 2005


5.4.1.  Normal Examples

      "g:h"           =  "g:h"
      "g"             =  "http://a/b/c/g"
      "./g"           =  "http://a/b/c/g"
      "g/"            =  "http://a/b/c/g/"
      "/g"            =  "http://a/g"
      "//g"           =  "http://g"
      "?y"            =  "http://a/b/c/d;p?y"
      "g?y"           =  "http://a/b/c/g?y"
      "#s"            =  "http://a/b/c/d;p?q#s"
      "g#s"           =  "http://a/b/c/g#s"
      "g?y#s"         =  "http://a/b/c/g?y#s"
      ";x"            =  "http://a/b/c/;x"
      "g;x"           =  "http://a/b/c/g;x"
      "g;x?y#s"       =  "http://a/b/c/g;x?y#s"
      ""              =  "http://a/b/c/d;p?q"
      "."             =  "http://a/b/c/"
      "./"            =  "http://a/b/c/"
      ".."            =  "http://a/b/"
      "../"           =  "http://a/b/"
      "../g"          =  "http://a/b/g"
      "../.."         =  "http://a/"
      "../../"        =  "http://a/"
      "../../g"       =  "http://a/g"

5.4.2.  Abnormal Examples

   Although the following abnormal examples are unlikely to occur in
   normal practice, all URI parsers should be capable of resolving them
   consistently.  Each example uses the same base as that above.

   Parsers must be careful in handling cases where there are more ".."
   segments in a relative-path reference than there are hierarchical
   levels in the base URI's path.  Note that the ".." syntax cannot be
   used to change the authority component of a URI.

      "../../../g"    =  "http://a/g"
      "../../../../g" =  "http://a/g"












Berners-Lee, et al.         Standards Track                    [Page 36]
```

<a id="source-page-37-part-1"></a>

## Source page 37

```text

RFC 3986                   URI Generic Syntax               January 2005


   Similarly, parsers must remove the dot-segments "." and ".." when
   they are complete components of a path, but not when they are only
   part of a segment.

      "/./g"          =  "http://a/g"
      "/../g"         =  "http://a/g"
      "g."            =  "http://a/b/c/g."
      ".g"            =  "http://a/b/c/.g"
      "g.."           =  "http://a/b/c/g.."
      "..g"           =  "http://a/b/c/..g"

   Less likely are cases where the relative reference uses unnecessary
   or nonsensical forms of the "." and ".." complete path segments.

      "./../g"        =  "http://a/b/g"
      "./g/."         =  "http://a/b/c/g/"
      "g/./h"         =  "http://a/b/c/g/h"
      "g/../h"        =  "http://a/b/c/h"
      "g;x=1/./y"     =  "http://a/b/c/g;x=1/y"
      "g;x=1/../y"    =  "http://a/b/c/y"

   Some applications fail to separate the reference's query and/or
   fragment components from the path component before merging it with
   the base path and removing dot-segments.  This error is rarely
   noticed, as typical usage of a fragment never includes the hierarchy
   ("/") character and the query component is not normally used within
   relative references.

      "g?y/./x"       =  "http://a/b/c/g?y/./x"
      "g?y/../x"      =  "http://a/b/c/g?y/../x"
      "g#s/./x"       =  "http://a/b/c/g#s/./x"
      "g#s/../x"      =  "http://a/b/c/g#s/../x"

   Some parsers allow the scheme name to be present in a relative
   reference if it is the same as the base URI scheme.  This is
   considered to be a loophole in prior specifications of partial URI
   [RFC1630].  Its use should be avoided but is allowed for backward
   compatibility.

      "http:g"        =  "http:g"         ; for strict parsers
                      /  "http://a/b/c/g" ; for backward compatibility










Berners-Lee, et al.         Standards Track                    [Page 37]
```

<a id="source-page-38-part-1"></a>

## Source page 38

```text

RFC 3986                   URI Generic Syntax               January 2005


6.  Normalization and Comparison

   One of the most common operations on URIs is simple comparison:
   determining whether two URIs are equivalent without using the URIs to
   access their respective resource(s).  A comparison is performed every
   time a response cache is accessed, a browser checks its history to
   color a link, or an XML parser processes tags within a namespace.
   Extensive normalization prior to comparison of URIs is often used by
   spiders and indexing engines to prune a search space or to reduce
   duplication of request actions and response storage.

   URI comparison is performed for some particular purpose.  Protocols
   or implementations that compare URIs for different purposes will
   often be subject to differing design trade-offs in regards to how
   much effort should be spent in reducing aliased identifiers.  This
   section describes various methods that may be used to compare URIs,
   the trade-offs between them, and the types of applications that might
   use them.

6.1.  Equivalence

   Because URIs exist to identify resources, presumably they should be
   considered equivalent when they identify the same resource.  However,
   this definition of equivalence is not of much practical use, as there
   is no way for an implementation to compare two resources unless it
   has full knowledge or control of them.  For this reason,
   determination of equivalence or difference of URIs is based on string
   comparison, perhaps augmented by reference to additional rules
   provided by URI scheme definitions.  We use the terms "different" and
   "equivalent" to describe the possible outcomes of such comparisons,
   but there are many application-dependent versions of equivalence.

   Even though it is possible to determine that two URIs are equivalent,
   URI comparison is not sufficient to determine whether two URIs
   identify different resources.  For example, an owner of two different
   domain names could decide to serve the same resource from both,
   resulting in two different URIs.  Therefore, comparison methods are
   designed to minimize false negatives while strictly avoiding false
   positives.

   In testing for equivalence, applications should not directly compare
   relative references; the references should be converted to their
   respective target URIs before comparison.  When URIs are compared to
   select (or avoid) a network action, such as retrieval of a
   representation, fragment components (if any) should be excluded from
   the comparison.





Berners-Lee, et al.         Standards Track                    [Page 38]
```

