# G Glossary (non-normative)

Source: [XML Schema Part 2: Datatypes](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/) · 2nd edition, 2004-10-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2004/REC-xmlschema-2-20041028/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](042-continuation-42.md) · [Next](044-h-references.md)

<a id="normative-glossary"></a>

## G Glossary (non-normative)

The listing below is for the benefit of readers of a printed version of this
document: it collects together all the definitions which appear in the
document above.

[atomic](004-2-3-lexical-space.md#dt-atomic)
:   **Atomic** datatypes
    are those having values which are regarded by this specification as
    being indivisible.

[base type](005-2-5-1-3-union-datatypes.md#dt-basetype)
:   Every
    datatype that is [·derived·](005-2-5-1-3-union-datatypes.md#dt-derived) by **restriction**
    is defined in terms of an existing datatype, referred to as its
    **base type**. **base type**s can be either
    [·primitive·](005-2-5-1-3-union-datatypes.md#dt-primitive) or [·derived·](005-2-5-1-3-union-datatypes.md#dt-derived).

[bounded](023-4-2-3-bounded.md#dt-bounded)
:   A datatype is **bounded**
    if its [·value space·](003-1-4-terminology.md#dt-value-space) has either an
    [·inclusive upper bound·](023-4-2-3-bounded.md#dt-inclusive-upper-bound) or an [·exclusive upper bound·](023-4-2-3-bounded.md#dt-exclusive-upper-bound)
    and either an [·inclusive lower bound·](023-4-2-3-bounded.md#dt-inclusive-lower-bound) or
    an
    [·exclusive lower bound·](023-4-2-3-bounded.md#dt-exclusive-lower-bound).

[built-in](006-2-5-2-3-derived-by-union.md#dt-built-in)
:   **Built-in**
    datatypes are those which are defined in this specification,
    and can be either [·primitive·](005-2-5-1-3-union-datatypes.md#dt-primitive) or
    [·derived·](005-2-5-1-3-union-datatypes.md#dt-derived);

[canonical lexical representation](004-2-3-lexical-space.md#dt-canonical-representation)
:   A **canonical lexical representation**
    is a set of literals from among the valid set of literals
    for a datatype such that there is a one-to-one mapping between literals
    in the **canonical lexical representation** and
    values in the [·value space·](003-1-4-terminology.md#dt-value-space).

[cardinality](023-4-2-3-bounded.md#dt-cardinality)
:   Every
    [·value space·](003-1-4-terminology.md#dt-value-space) has associated with it the concept of
    **cardinality**. Some [·value space·](003-1-4-terminology.md#dt-value-space)s
    are finite, some are countably infinite while still others could
    conceivably be uncountably infinite (although no [·value space·](003-1-4-terminology.md#dt-value-space)
    defined by this specification is uncountable infinite). A datatype is
    said to have the cardinality of its
    [·value space·](003-1-4-terminology.md#dt-value-space).

[comparable](022-4-1-6-simple-type-definition-for-anysimpletype.md#dt-comparable)
:   otherwise they are **comparable**.

[conformance to the XML Representation of Schemas](033-5-conformance.md#dt-interchange)
:   Processors which accept schemas in the form of XML documents as described
    in [XML Representation of Simple Type Definition Schema Components (§4.1.2)](018-4-datatype-components.md#xr-defn) (and other relevant portions of
    [Datatype components (§4)](018-4-datatype-components.md#datatype-components)) are additionally said to provide
    **conformance to the XML Representation of Schemas**,
    and [·must·](003-1-4-terminology.md#dt-must), when processing schema documents, completely and
    correctly implement all
    [·Schema Representation Constraint·](003-1-4-terminology.md#dt-src)s
    in this specification, and [·must·](003-1-4-terminology.md#dt-must) adhere exactly to the
    specifications in [XML Representation of Simple Type Definition Schema Components (§4.1.2)](018-4-datatype-components.md#xr-defn) (and other relevant portions of
    [Datatype components (§4)](018-4-datatype-components.md#datatype-components)) for mapping
    the contents of such
    documents to [schema components](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/structures.html#key-component)
    for use in validation.

[constraining facet](004-2-3-lexical-space.md#dt-constraining-facet)
:   A
    **constraining facet** is an optional property that can be
    applied to a datatype to constrain its [·value space·](003-1-4-terminology.md#dt-value-space).

[Constraint on Schemas](003-1-4-terminology.md#dt-cos)
:   **Constraint on Schemas**

[datatype](003-1-4-terminology.md#dt-datatype)
:   In this specification,
    a **datatype** is a 3-tuple, consisting of
    a) a set of distinct values, called its [·value space·](003-1-4-terminology.md#dt-value-space),
    b) a set of lexical representations, called its
    [·lexical space·](004-2-3-lexical-space.md#dt-lexical-space), and c) a set of [·facet·](004-2-3-lexical-space.md#dt-facet)s
    that characterize properties of the [·value space·](003-1-4-terminology.md#dt-value-space),
    individual values or lexical items.

[derived](005-2-5-1-3-union-datatypes.md#dt-derived)
:   **Derived**
    datatypes are those that are defined in terms of other datatypes.

[error](003-1-4-terminology.md#dt-error)
:   **error**

[exclusive lower bound](023-4-2-3-bounded.md#dt-exclusive-lower-bound)
:   A value *l* in an [·ordered·](022-4-1-6-simple-type-definition-for-anysimpletype.md#dt-ordered)
     [·value space·](003-1-4-terminology.md#dt-value-space) *L*
    is said to be an **exclusive lower bound** of a
    [·value space·](003-1-4-terminology.md#dt-value-space) *V*
    (where *V* is a subset of *L*)
    if for all *v* in *V*,
    *l* < *v*.

[exclusive upper bound](023-4-2-3-bounded.md#dt-exclusive-upper-bound)
:   A value *u* in an [·ordered·](022-4-1-6-simple-type-definition-for-anysimpletype.md#dt-ordered)
     [·value space·](003-1-4-terminology.md#dt-value-space) *U*
    is said to be an **exclusive upper bound** of a
    [·value space·](003-1-4-terminology.md#dt-value-space) *V*
    (where *V* is a subset of *U*)
    if for all *v* in *V*,
    *u* > *v*.

[facet](004-2-3-lexical-space.md#dt-facet)
:   A **facet** is a single
    defining aspect of a [·value space·](003-1-4-terminology.md#dt-value-space). Generally
    speaking, each facet characterizes a [·value space·](003-1-4-terminology.md#dt-value-space)
    along independent axes or dimensions.

[for compatibility](003-1-4-terminology.md#dt-compatibility)
:   for compatibility

[fundamental facet](004-2-3-lexical-space.md#dt-fundamental-facet)
:   A **fundamental facet** is an abstract property which
    serves to semantically characterize the values in a
    [·value space·](003-1-4-terminology.md#dt-value-space).

[inclusive lower bound](023-4-2-3-bounded.md#dt-inclusive-lower-bound)
:   A value *l* in an [·ordered·](022-4-1-6-simple-type-definition-for-anysimpletype.md#dt-ordered)
     [·value space·](003-1-4-terminology.md#dt-value-space) *L*
    is said to be an **inclusive lower bound** of a
    [·value space·](003-1-4-terminology.md#dt-value-space) *V*
    (where *V* is a subset of *L*)
    if for all *v* in *V*,
    *l* <= *v*.

[inclusive upper bound](023-4-2-3-bounded.md#dt-inclusive-upper-bound)
:   A value *u* in an [·ordered·](022-4-1-6-simple-type-definition-for-anysimpletype.md#dt-ordered)
     [·value space·](003-1-4-terminology.md#dt-value-space) *U*
    is said to be an **inclusive upper bound** of a
    [·value space·](003-1-4-terminology.md#dt-value-space) *V*
    (where *V* is a subset of *U*)
    if for all *v* in *V*,
    *u* >= *v*.

[incomparable](022-4-1-6-simple-type-definition-for-anysimpletype.md#dt-incomparable)
:   When *a <> b*, *a* and *b* are **incomparable**,

[itemType](004-2-3-lexical-space.md#dt-itemType)
:   The [·atomic·](004-2-3-lexical-space.md#dt-atomic) or [·union·](004-2-3-lexical-space.md#dt-union)
    datatype that participates in the definition of a [·list·](004-2-3-lexical-space.md#dt-list) datatype
    is known as the **itemType** of that [·list·](004-2-3-lexical-space.md#dt-list) datatype.

[lexical space](004-2-3-lexical-space.md#dt-lexical-space)
:   A
    **lexical space** is the set of valid *literals*
    for a datatype.

[list](004-2-3-lexical-space.md#dt-list)
:   **List**
    datatypes are those having values each of which consists of a
    finite-length (possibly empty) sequence of values of an
    [·atomic·](004-2-3-lexical-space.md#dt-atomic) datatype.

[match](003-1-4-terminology.md#dt-match)
:   **match**

[may](003-1-4-terminology.md#dt-may)
:   **may**

[memberTypes](005-2-5-1-3-union-datatypes.md#dt-memberTypes)
:   The datatypes that participate in the
    definition of a [·union·](004-2-3-lexical-space.md#dt-union) datatype are known as the
    **memberTypes** of that [·union·](004-2-3-lexical-space.md#dt-union) datatype.

[minimally conforming](033-5-conformance.md#dt-minimally-conforming)
:   **Minimally conforming** processors [·must·](003-1-4-terminology.md#dt-must)
    completely and correctly implement the [·Constraint on Schemas·](003-1-4-terminology.md#dt-cos) and
    [·Validation Rule·](003-1-4-terminology.md#dt-cvc)
    .

[must](003-1-4-terminology.md#dt-must)
:   **must**

[non-numeric](024-4-2-5-numeric.md#dt-non-numeric)
:   A datatype whose values
    are not [·numeric·](024-4-2-5-numeric.md#dt-numeric) is said to be
    **non-numeric**.

[numeric](024-4-2-5-numeric.md#dt-numeric)
:   A datatype is said to be
    **numeric** if its values are conceptually quantities (in some
    mathematical number system).

[order-relation](022-4-1-6-simple-type-definition-for-anysimpletype.md#dt-order-relation)
:   An
    **order relation** on a [·value space·](003-1-4-terminology.md#dt-value-space)
    is a mathematical relation that imposes a
    [·total order·](022-4-1-6-simple-type-definition-for-anysimpletype.md#dt-total-order) or a [·partial order·](022-4-1-6-simple-type-definition-for-anysimpletype.md#dt-partial-order) on the
    members of the [·value space·](003-1-4-terminology.md#dt-value-space).

[ordered](022-4-1-6-simple-type-definition-for-anysimpletype.md#dt-ordered)
:   A
    [·value space·](003-1-4-terminology.md#dt-value-space), and hence a datatype, is said to be
    **ordered** if there exists an
    [·order-relation·](022-4-1-6-simple-type-definition-for-anysimpletype.md#dt-order-relation) defined for that
    [·value space·](003-1-4-terminology.md#dt-value-space).

[partial order](022-4-1-6-simple-type-definition-for-anysimpletype.md#dt-partial-order)
:   A **partial order** is an [·order-relation·](022-4-1-6-simple-type-definition-for-anysimpletype.md#dt-order-relation)
    that is **irreflexive**, **asymmetric** and
    **transitive**.

[primitive](005-2-5-1-3-union-datatypes.md#dt-primitive)
:   **Primitive**
    datatypes are those that are not defined in terms of other datatypes;
    they exist *ab initio*.

[regular expression](039-e-1-algorithm.md#dt-regex)
:   A
    **regular expression** is composed from zero or more
    [·branch·](039-e-1-algorithm.md#dt-branch)es, separated by `|` characters.

[restriction](005-2-5-1-3-union-datatypes.md#dt-restriction)
:   A datatype is said to be
    [·derived·](005-2-5-1-3-union-datatypes.md#dt-derived) by **restriction** from another datatype
    when values for zero or more [·constraining facet·](004-2-3-lexical-space.md#dt-constraining-facet)s are specified
    that serve to constrain its [·value space·](003-1-4-terminology.md#dt-value-space) and/or its
    [·lexical space·](004-2-3-lexical-space.md#dt-lexical-space) to a subset of those of its
    [·base type·](005-2-5-1-3-union-datatypes.md#dt-basetype).

