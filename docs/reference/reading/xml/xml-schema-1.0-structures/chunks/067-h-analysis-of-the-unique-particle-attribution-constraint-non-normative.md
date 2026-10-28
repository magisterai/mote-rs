# H Analysis of the Unique Particle Attribution Constraint (non-normative)

Source: [XML Schema Part 1: Structures](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/) · 2nd edition, 2004-10-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](066-continuation-66.md)




<a id="non-ambig"></a>

## H Analysis of the Unique Particle Attribution Constraint (non-normative)

A specification of the import of [Unique Particle Attribution (§3.8.6)](034-3-8-5-model-group-information-set-contributions.md#cos-nonambig) which does
not appeal to a processing model is difficult. What follows is intended as
guidance, without claiming to be complete.

<a id="key-overlap"></a>

[Definition:]Two non-group
particles **overlap** if

- They are both element declaration particles whose declarations have the
  same [{name}](013-3-3-element-declarations.md#e-name) and [{target namespace}](013-3-3-element-declarations.md#e-target_namespace).

or

- They are both element declaration particles one of whose [{name}](013-3-3-element-declarations.md#e-name) and [{target namespace}](013-3-3-element-declarations.md#e-target_namespace) are the same as those of an element declaration in the
  other's [·substitution group·](019-continuation-19.md#key-eq).

or

- They are both wildcards, and the intensional intersection of their [{namespace constraint}](038-3-10-wildcards.md#namespace_constraint)s as defined
  in [Attribute Wildcard Intersection (§3.10.6)](039-3-10-4-wildcard-validation-rules.md#cos-aw-intersect) is not the empty set.

or

- One is a wildcard and the other an element declaration, and the
  [{target namespace}](013-3-3-element-declarations.md#e-target_namespace) of any
  member of its [·substitution group·](019-continuation-19.md#key-eq) is [·valid·](004-2-1-overview-of-xml-schema.md#key-vn) with respect to the [{namespace constraint}](038-3-10-wildcards.md#namespace_constraint) of the wildcard.

A content model will violate the unique attribution constraint if it
contains two particles which [·overlap·](067-h-analysis-of-the-unique-particle-attribution-constraint-non-normative.md#key-overlap) and which either

- are both in the [{particles}](033-3-7-6-constraints-on-model-group-definition-schema-components.md#particles) of a *choice* or
  *all* group

or

- may [·validate·](004-2-1-overview-of-xml-schema.md#key-vn) adjacent information items and the first has
  [{min occurs}](035-3-9-particles.md#p-min_occurs) less than [{max occurs}](035-3-9-particles.md#p-max_occurs).

Two particles may [·validate·](004-2-1-overview-of-xml-schema.md#key-vn) adjacent information items if they are
separated by at most epsilon transitions in the most obvious transcription of a
content model into a finite-state automaton.

A precise formulation of this constraint can also be offered in terms of
operations on finite-state automaton: transcribe the content model into an
automaton in the usual way using epsilon transitions for optionality and
unbounded maxOccurs, unfolding other numeric occurrence ranges and treating the heads of substitution groups as if
they were choices over all elements in the group, *but* using not
element QNames as transition labels, but rather pairs of element QNames and
positions in the model. Determinize this automaton, treating wildcard transitions as opaque. Now replace all
QName+position transition labels with the element QNames alone. If the result has any states
with two or more identical-QName-labeled transitions from it, or a
QName-labeled transition and a wildcard transition which subsumes it, or two
wildcard transitions whose intentional intersection is non-empty, the model does not
satisfy the Unique Attribution constraint.

<a id="nonnormative-references"></a>

## I References (non-normative)

<a id="ref-dcd"></a>

DCD
:   *Document
    Content Description for XML (DCD)*, Tim Bray et al., eds., W3C, 10 August 1998.
    See [http://www.w3.org/TR/1998/NOTE-dcd-19980731](https://www.w3.org/TR/1998/NOTE-dcd-19980731)

<a id="ref-ddml"></a>

DDML
:   *Document
    Definition Markup Language*, Ronald Bourret, John Cowan, Ingo Macherius,
    Simon St. Laurent, eds., W3C, 19 January 1999. See
    [http://www.w3.org/TR/1999/NOTE-ddml-19990119](https://www.w3.org/TR/1999/NOTE-ddml-19990119)

<a id="ref-sox"></a>

SOX
:   *Schema for
    Object-oriented XML*, Andrew Davidson et al., eds., W3C, 1998. See
    [http://www.w3.org/1999/07/NOTE-SOX-19990730/](https://www.w3.org/1999/07/NOTE-SOX-19990730/)

<a id="ref-sox-1.1"></a>

SOX-2
:   *Schema
    for Object-oriented XML*, Version 2.0, Andrew Davidson, et al., W3C, 30
    July 1999. See
    [http://www.w3.org/TR/NOTE-SOX/](https://www.w3.org/TR/NOTE-SOX/)

<a id="ref-xdr"></a>

XDR
:   *XML-Data
    Reduced*, Charles Frankston and Henry S. Thompson, 3 July 1998. See
    [http://www.ltg.ed.ac.uk/~ht/XMLData-Reduced.htm](http://www.ltg.ed.ac.uk/~ht/XMLData-Reduced.htm)

<a id="bib-expo"></a>

XML Schema: Primer
:   *XML Schema Part 0:
    Primer*, David C. Fallside, ed., W3C, 2 May 2001. See [http://www.w3.org/TR/2004/REC-xmlschema-0-20041028/primer.html](https://www.w3.org/TR/2004/REC-xmlschema-0-20041028/primer.html)

<a id="ref-xml-data"></a>

XML-Data
:   *XML-Data*, Andrew Layman et al., W3C, 05 January 1998. See
    [http://www.w3.org/TR/1998/NOTE-XML-data-0105/](https://www.w3.org/TR/1998/NOTE-XML-data-0105/)

<a id="acknowledgments"></a>

## J Acknowledgements (non-normative)

The following
contributed material to the first edition of this specification:

> David Fallside, IBM  
> Scott Lawrence, Agranat Systems  
> Andrew Layman, Microsoft  
> Eve L. Maler, Sun Microsystems  
> Asir S. Vedamuthu, webMethods, Inc

The editors acknowledge the members of the XML Schema Working Group, the members of other W3C Working Groups, and industry experts in other
forums who have contributed directly or indirectly to the process or content of
creating this document. The Working Group is particularly grateful to Lotus
Development Corp. and IBM for providing teleconferencing facilities.

At the time the first edition of this
specification was published, the members of the XML Schema Working Group
were:

- Jim Barnette, Defense Information Systems Agency (DISA)
- Paul V. Biron, Health Level Seven
- Don Box, DevelopMentor
- Allen Brown, Microsoft
- Lee Buck, TIBCO Extensibility
- Charles E. Campbell, Informix
- Wayne Carr, Intel
- Peter Chen, Bootstrap Alliance and LSU
- David Cleary, Progress Software
- Dan Connolly, W3C (*staff contact*)
- Ugo Corda, Xerox
- Roger L. Costello, MITRE
- Haavard Danielson, Progress Software
- Josef Dietl, Mozquito Technologies
- David Ezell, Hewlett-Packard Company
- Alexander Falk, Altova GmbH
- David Fallside, IBM
- Dan Fox, Defense Logistics Information Service (DLIS)
- Matthew Fuchs, Commerce One
- Andrew Goodchild, Distributed Systems Technology Centre (DSTC Pty Ltd)
- Paul Grosso, Arbortext, Inc
- Martin Gudgin, DevelopMentor
- Dave Hollander, Contivo, Inc (*co-chair*)
- Mary Holstege, Invited Expert
- Jane Hunter, Distributed Systems Technology Centre (DSTC Pty Ltd)
- Rick Jelliffe, Academia Sinica
- Simon Johnston, Rational Software
- Bob Lojek, Mozquito Technologies
- Ashok Malhotra, Microsoft
- Lisa Martin, IBM
- Noah Mendelsohn, Lotus Development Corporation
- Adrian Michel, Commerce One
- Alex Milowski, Invited Expert
- Don Mullen, TIBCO Extensibility
- Dave Peterson, Graphic Communications Association
- Jonathan Robie, Software AG
- Eric Sedlar, Oracle Corp.
- C. M. Sperberg-McQueen, W3C (*co-chair*)
- Bob Streich, Calico Commerce
- William K. Stumbo, Xerox
- Henry S. Thompson, University of Edinburgh
- Mark Tucker, Health Level Seven
- Asir S. Vedamuthu, webMethods, Inc
- Priscilla Walmsley, XMLSolutions
- Norm Walsh, Sun Microsystems
- Aki Yoshida, SAP AG
- Kongyi Zhou, Oracle Corp.

The XML Schema Working Group has benefited in its work from the
participation and contributions of a number of people not currently
members of the Working Group, including
in particular those named below. Affiliations given are those current at
the time of their work with the WG.

- Paula Angerstein, Vignette Corporation
- David Beech, Oracle Corp.
- Gabe Beged-Dov, Rogue Wave Software
- Greg Bumgardner, Rogue Wave Software
- Dean Burson, Lotus Development Corporation
- Mike Cokus, MITRE
- Andrew Eisenberg, Progress Software
- Rob Ellman, Calico Commerce
- George Feinberg, Object Design
- Charles Frankston, Microsoft
- Ernesto Guerrieri, Inso
- Michael Hyman, Microsoft
- Renato Iannella, Distributed Systems Technology Centre (DSTC Pty Ltd)
- Dianne Kennedy, Graphic Communications Association
- Janet Koenig, Sun Microsystems
- Setrag Khoshafian, Technology Deployment International (TDI)
- Ara Kullukian, Technology Deployment International (TDI)
- Andrew Layman, Microsoft
- Dmitry Lenkov, Hewlett-Packard Company
- John McCarthy, Lawrence Berkeley National Laboratory
- Murata Makoto, Xerox
- Eve Maler, Sun Microsystems
- Murray Maloney, Muzmo Communication, acting for Commerce One
- Chris Olds, Wall Data
- Frank Olken, Lawrence Berkeley National Laboratory
- Shriram Revankar, Xerox
- Mark Reinhold, Sun Microsystems
- John C. Schneider, MITRE
- Lew Shannon, NCR
- William Shea, Merrill Lynch
- Ralph Swick, W3C
- Tony Stewart, Rivcom
- Matt Timmermans, Microstar
- Jim Trezzo, Oracle Corp.
- Steph Tryphonas, Microstar

The lists given above pertain to the first edition.
At the time work on this second edition was completed,
the membership of the Working Group was:

- Leonid Arbouzov, Sun Microsystems
- Jim Barnette, Defense Information Systems Agency (DISA)
- Paul V. Biron, Health Level Seven
- Allen Brown, Microsoft
- Charles E. Campbell, Invited expert
- Peter Chen, Invited expert
- Tony Cincotta, NIST
- David Ezell, National Association of Convenience Stores
- Matthew Fuchs, Invited expert
- Sandy Gao, IBM
- Andrew Goodchild, Distributed Systems Technology Centre (DSTC Pty Ltd)
- Xan Gregg, Invited expert
- Mary Holstege, Mark Logic
- Mario Jeckle, DaimlerChrysler
- Marcel Jemio, Data Interchange Standards Association
- Kohsuke Kawaguchi, Sun Microsystems
- Ashok Malhotra, Invited expert
- Lisa Martin, IBM
- Jim Melton, Oracle Corp
- Noah Mendelsohn, IBM
- Dave Peterson, Invited expert
- Anli Shundi, TIBCO Extensibility
- C. M. Sperberg-McQueen, W3C (*co-chair*)
- Hoylen Sue, Distributed Systems Technology Centre (DSTC Pty Ltd)
- Henry S. Thompson, University of Edinburgh
- Asir S. Vedamuthu, webMethods, Inc
- Priscilla Walmsley, Invited expert
- Kongyi Zhou, Oracle Corp.

We note with sadness the accidental death of Mario Jeckle
shortly after the completion of work on this document.
In addition to those named above, several
people served on the Working Group during the development
of this second edition:

- Oriol Carbo, University of Edinburgh
- Tyng-Ruey Chuang, Academia Sinica
- Joey Coyle, Health Level 7
- Tim Ewald, DevelopMentor
- Nelson Hung, Corel
- Melanie Kudela, Uniform Code Council
- Matthew MacKenzie, XML Global
- Cliff Schmidt, Microsoft
- John Stanton, Defense Information Systems Agency
- John Tebbutt, NIST
- Ross Thompson, Contivo
- Scott Vorthmann, TIBCO Extensibility
