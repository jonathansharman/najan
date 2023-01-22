# Determiners

A determiner phrase consists of a determiner, an optional pronoun to bind, and a
verb phrase complement; it signifies some instance or set of entities such that
the complement holds true.

In the case of stative verb complements like <naj>tab</naj> ⟨tab⟩ ("be a
father"), the noun phrase implicitly acts as the subject.

<div class="caption">"to be a happy father"</div>

| | | |
| :-- | :-- | :-- |
| <naj>nʊ</naj>  | <naj>tayn</naj> | <naj>tab</naj> |
|      nʊ        |      tayn       |      tab       |
|      such-that |      be-happy   |      be-father |

<div class="caption">"a happy father"</div>

| | | | |
| :-- | :-- | :-- | :-- |
| <naj>a</naj>     | <naj>nʊ</naj>  | <naj>tayn</naj> | <naj>tab</naj> |
|      a           |      nʊ        |      tayn       |      tab       |
|      a-ᴅᴇᴛ       |      such-that |      be-happy   |      be-father |

For dynamic verb complements, the noun phrase by default refers to the action
itself (and not for instance to the agent of the action).

<div class="caption">"an action"</div>

| | |
| :-- | :-- |
| <naj>a</naj>     | <naj>pɪn</naj> |
|      a           |      pɪn       |
|      a-ᴅᴇᴛ       |      do        |

TODO: Organize specific determiners into subsections

The universal distributive determiner <naj>u</naj> ⟨u⟩ expresses that its
immediately enclosing clause holds true individually for each instance of its
complement, functioning similarly to "each" or "every". This determiner is
unique in that it only binds pronouns for the duration of the containing clause.

<div class="caption">"every language changes"</div>

| | | | | | |
| :-- | :-- | :-- | :-- | :-- | :-- |
| <naj>je</naj> | <naj>rʊ</naj> | <naj>u</naj>  | <naj>vix</naj> | <naj>fxe</naj> | <naj>cay</naj> |
|      je       |      rʊ       |      u        |      vix       |      fxe       |      cay       |
|      ᴵᴺᴰ      |      ᴾ        |      each-ᴅᴇᴛ |      language  |      ʜᴀʙ       |      change    |

TODO: Examples illustrating why the universal and existential determiners vary
in binding scope as they do. "Every man X has a horse Y; Y is a good horse"
doesn't make sense. "A man X has a horse Y; Y is a good horse" does.


Generally, bound pronouns remain bound until rebound to a new
referent. However, any pronouns bound using the universal determiner
<naj>u</naj> ⟨u⟩—or in any subsequent quantified term within the same
clause—become unbound at the end of the current clause.

\paragraph{Existential} The existential determiner \trans{a} binds pronouns to
individual instances of a complement, similar to many uses of English
"a"/"an". When it binds multiple pronouns, each signifies a unique instance.
Pronouns bound by separate existential determiners applied to the same
complement may or may not share a referent.

\example{Existential determiner}
{I have a dog.}
{\transLine{ya lut ko a zho nhiruh}}

\todo{ Rework this old example, talking about shifting modifiers around instead.
	Also include an example using a specialized "for" modifier, for when the
	pronouns need to be defined together but used separately? }

\example{Using forward assignment to change semantics}{
	Every person has an ancestor. \\
	$\Downarrow$ \\
	For each person, there's a person that's an ancestor to the first. \\
}{
	\transLine{u zho mil ya nhinh a co mil OF runat zho} \\
	$\Downarrow$ \\
	\transLine{u zho mil a co mil ya nhinh co OF runat zho} \\
}

\paragraph{Definite} When the set of entities an unquantified term refers to is
a singleton, i.e. when it has only one instance, the definitive determiner
\trans{i} can be used to refer to that single instance, functioning much the
same as the English determiner "the" in many cases.

\example{Definite determiner}{
	A dog wagged its tail. \\
	$\Downarrow$ \\
	Before now, a dog wagged the tail belonging to it.
}{
	\transLine{the va threre a zho nhiruh i co sex zho shuht}
}

\paragraph{Demonstrative} Demonstrative determiners function just like the
definite determiner except that the complement need not be a singleton. Instead,
the particular instance of the complement is understood by context, e.g. by
physical gesturing or by common understanding. Najan has proximal
(\trans{ihla}), medial (\trans{aya}), and distal (\trans{owa}) demonstrative
determiners, based on the distance of the referent from the speaker in space and
time. These correspond roughly to English "this", "that", and "yonder",
respectively. The concept of distance with regard to demonstrative determiners
can be more abstract than just spatiotemporal distance to a physical object;
e.g. it could be a measure of the relevance of the complement to the present
conversation.

\paragraph{Indefinite} The indefinite determiner \trans{na} acts like the
English word "any", meaning a single instance of the complement where the
particular instance does not matter. As with the existential determiner, it can
bind multiple pronouns at once, in which case each referent is unique. Note that
many English sentences using "a"/"an" are best translated using \trans{na}
rather than \trans{a}.

\example{Indefinite determiner}{
	I want a book.
}{
	\transLine{ka na zho suaysh zihm ko}
	\\
	acting-on any pron. book want I
}

\example{Indefinite determiner binding two pronouns}{
	I want two books.
}{
	\transLine{wa na zho co suaysh ka sem zho co zihm ko}
	\\
	forward-assignment any pron.-1 pron.-2 book acting-on and pron.-1 pron.-2 want I
}

\todo{ This would probably be better translated as "I want any books numbering
	two" unless the specific two books are going to be referenced individually
	later. Might want to come up with a better example or add a sentence using
	the books. }
