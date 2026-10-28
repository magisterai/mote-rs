# 3.10.4 Wildcard Validation Rules

Source: [XML Schema Part 1: Structures](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/) · 2nd edition, 2004-10-28.

[Document index](../README.md) · [Topic guide](../../../../README.md) · [Publisher](https://www.w3.org/TR/2004/REC-xmlschema-1-20041028/).

HTML normalized to Markdown. Complex tables and mathematical markup retain HTML; illustrations are stored locally.

[Previous](038-3-10-wildcards.md) · [Next](040-3-11-identity-constraint-definitions.md)

In addition to the conditions imposed on [<any>](038-3-10-wildcards.md#element-any) element
information items by the schema for schemas, the corresponding particle and model group must satisfy the conditions set
out in [Constraints on Model Group Schema Components (§3.8.6)](034-3-8-5-model-group-information-set-contributions.md#coss-modelGroup) and [Constraints on Particle Schema Components (§3.9.6)](036-3-9-5-particle-information-set-contributions.md#coss-particle).

<a id="d0e12802"></a>

#### 3.10.4 Wildcard Validation Rules

<a id="cvc-wildcard"></a>

**Validation Rule: Item Valid (Wildcard)**  

For an element or attribute information item to be locally [·valid·](004-2-1-overview-of-xml-schema.md#key-vn) with respect to a wildcard
constraint
its [[namespace name]](https://www.w3.org/TR/xml-infoset/#infoitem.element) must be [·valid·](004-2-1-overview-of-xml-schema.md#key-vn) with respect to the wildcard constraint, as defined in [Wildcard allows Namespace Name (§3.10.4)](039-3-10-4-wildcard-validation-rules.md#cvc-wildcard-namespace).When this constraint applies
the appropriate **case** among the following
must be true:

1 **If** [{process contents}](038-3-10-wildcards.md#process_contents) is *lax*, **then** the item has no [·context-determined declaration·](011-3-2-4-attribute-declaration-validation-rules.md#key-dd) with respect to [Assessment Outcome (Element) (§3.3.5)](017-3-3-5-element-declaration-information-set-contributions.md#sic-e-outcome), [Schema-Validity Assessment (Element) (§3.3.4)](017-3-3-5-element-declaration-information-set-contributions.md#cvc-assess-elt) and [Schema-Validity Assessment (Attribute) (§3.2.4)](011-3-2-4-attribute-declaration-validation-rules.md#cvc-assess-attr).

2 **If** [{process contents}](038-3-10-wildcards.md#process_contents) is *strict*, **then** the item's [·context-determined declaration·](011-3-2-4-attribute-declaration-validation-rules.md#key-dd) is *mustFind*.

3 **If** [{process contents}](038-3-10-wildcards.md#process_contents) is *skip*, **then** the item's [·context-determined declaration·](011-3-2-4-attribute-declaration-validation-rules.md#key-dd) is *skip*.

<a id="cvc-wildcard-namespace"></a>

**Validation Rule: Wildcard allows Namespace Name**  

For a value which is either a namespace name or [·absent·](008-3-schema-component-details.md#key-null) to be [·valid·](004-2-1-overview-of-xml-schema.md#key-vn) with respect to a wildcard constraint (the
value of a [{namespace constraint}](038-3-10-wildcards.md#namespace_constraint))
**one** of the following must be true:

1 The constraint must be *any*.

2 **All** of the following must be true:

2.1 The constraint is a pair of *not* and a namespace name or
[·absent·](008-3-schema-component-details.md#key-null) (

<a id="key-nst"></a>

[Definition:]call this the **namespace test**).

2.2 The value must not be identical to the [·namespace test·](039-3-10-4-wildcard-validation-rules.md#key-nst).

2.3 The value must not be [·absent·](008-3-schema-component-details.md#key-null).

3 The constraint is a set, and the value is identical to one of the members of the set.

<a id="d0e12923"></a>

#### 3.10.5 Wildcard Information Set Contributions

None as such.

<a id="coss-wildcard"></a>

#### 3.10.6 Constraints on Wildcard Schema Components

All wildcards (see [Wildcards (§3.10)](038-3-10-wildcards.md#Wildcards)) must satisfy the following constraint.

<a id="w-props-correct"></a>

**Schema Component Constraint: Wildcard Properties Correct**  

The values of the properties of a wildcard must be as described in
the property tableau in
[The Wildcard Schema Component (§3.10.1)](038-3-10-wildcards.md#Wildcard_details), modulo the impact of [Missing Sub-components (§5.3)](058-5-3-missing-sub-components.md#conformance-missing).

The following constraints define a relation appealed to elsewhere in this specification.

<a id="cos-ns-subset"></a>

**Schema Component Constraint: Wildcard Subset**  

For a namespace constraint (call it **sub**) to be an intensional subset of
another namespace constraint (call it **super**)
**one** of the following must be true:

1 **super** must be *any*.

2 **All** of the following must be true:

2.1 **sub** must be a pair of *not* and
a value
(a namespace name or
[·absent·](008-3-schema-component-details.md#key-null)).

2.2 **super** must be a pair of *not* and the same value.

3 **All** of the following must be true:

3.1 **sub** must be a set whose members are either namespace names or
[·absent·](008-3-schema-component-details.md#key-null).

3.2 **One** of the following must be true:

3.2.1 **super** must be the same set or a superset thereof.

3.2.2 **super** must be a pair of *not* and a value (a namespace name or
[·absent·](008-3-schema-component-details.md#key-null))
and neither that value nor [·absent·](008-3-schema-component-details.md#key-null) must be in **sub**'s set.

<a id="cos-aw-union"></a>

**Schema Component Constraint: Attribute Wildcard Union**  

For a wildcard's [{namespace constraint}](038-3-10-wildcards.md#namespace_constraint) value to be the intensional
union of two other such values (call them **O1** and **O2**):
the appropriate **case** among the following
must be true:

1 **If** **O1** and **O2** are the same value, **then** that value must be the value.

2 **If** either **O1** or **O2** is *any*, **then** *any* must be the value.

3 **If** both **O1** and **O2** are sets of (namespace names
or [·absent·](008-3-schema-component-details.md#key-null)), **then** the union of those sets must be the value.

4 **If** the two are negations of different values (namespace names or [·absent·](008-3-schema-component-details.md#key-null)), **then** a pair of *not* and [·absent·](008-3-schema-component-details.md#key-null) must be the value.

5 **If** either **O1** or **O2** is a pair of *not*
and a namespace name and the other is a set of (namespace names or [·absent·](008-3-schema-component-details.md#key-null)) (call this set **S**), **then** 
The appropriate **case** among the following
must be true:

5.1 **If** the set **S** includes both the negated namespace name and [·absent·](008-3-schema-component-details.md#key-null), **then** *any* must be the value.

5.2 **If** the set **S** includes the negated namespace name but not [·absent·](008-3-schema-component-details.md#key-null), **then** a pair of *not* and [·absent·](008-3-schema-component-details.md#key-null) must be the value.

5.3 **If** the set **S** includes [·absent·](008-3-schema-component-details.md#key-null)
but not the negated namespace name, **then** the union is not expressible.

5.4 **If** the set **S** does not include either the negated namespace
name or [·absent·](008-3-schema-component-details.md#key-null), **then** whichever of **O1** or **O2** is a pair of *not*
and a namespace name must be the value.

6 **If** either **O1** or **O2** is a pair of *not*
and [·absent·](008-3-schema-component-details.md#key-null) and the other is a set of
(namespace names or [·absent·](008-3-schema-component-details.md#key-null)) (again, call this
set **S**), **then** 
The appropriate **case** among the following
must be true:

6.1 **If** the set **S** includes [·absent·](008-3-schema-component-details.md#key-null), **then** *any* must be the value.

6.2 **If** the set **S** does not include [·absent·](008-3-schema-component-details.md#key-null), **then** a pair of *not* and [·absent·](008-3-schema-component-details.md#key-null) must be the value.

In the case where there are more than two values, the intensional
union is determined by identifying the intensional union of two
of the values as above, then the intensional union of that value with
the third (providing the first union was expressible), and so on as required.

<a id="cos-aw-intersect"></a>

**Schema Component Constraint: Attribute Wildcard Intersection**  

For a wildcard's [{namespace constraint}](038-3-10-wildcards.md#namespace_constraint) value to be the intensional
intersection of two other such values (call them **O1** and **O2**):
the appropriate **case** among the following
must be true:

1 **If** **O1** and **O2** are the same value, **then** that value must be the value.

2 **If** either **O1** or **O2** is *any*, **then** the
other must be the value.

3 **If** either **O1** or **O2** is a pair of *not*
and a value (a namespace name or [·absent·](008-3-schema-component-details.md#key-null)) and the other is a set of (namespace names or [·absent·](008-3-schema-component-details.md#key-null)), **then** that set,
minus the negated value if it was in the set, minus [·absent·](008-3-schema-component-details.md#key-null) if it was in the set, must be the value.

4 **If** both **O1** and **O2** are sets of (namespace names
or [·absent·](008-3-schema-component-details.md#key-null)), **then** the intersection of those sets must be the value.

5 **If** the two are negations of different namespace names, **then** the intersection is not expressible.

6 **If** the one is a negation of a namespace name and the other is a
negation of [·absent·](008-3-schema-component-details.md#key-null), **then** the one which is the negation of a namespace name must be the value.

In the case where there are more than two values, the intensional
intersection is determined by identifying the intensional intersection of two
of the values as above, then the intensional intersection of that value with
the third (providing the first intersection was expressible), and so on as required.

