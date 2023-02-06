# Determiner Phrases

A **determiner phrase** consists of a determiner, an optional pronoun to bind,
and a stative verb phrase complement. The phrase signifies some entity or set of
entities such that the complement holds true with those entities acting as the
subject.

Combining the existential determiner {za} with the stative verb {tab} ("to be a
father") forms the determiner phrase {za tab}, which means "an entity that is a
father" or simply "a father".

Binding a pronoun in a determiner phrase allows referring back to it in a
concise and unambiguous way.

<gloss>
{ci} | {gi} | {paw} | {dzað}   | {rʊ} | {paw} | {khɪtc}
ᴬ    | the  | it    | be-child | ᴾ    | it    | hurt
The child hurt itself.
</gloss>

To instantiate the action or state of a verb itself, we can first
[stativize](./stativization.md) the verb. Compare {wo tayn} ("all that are
happy"), to {wo djʊ tayn} ("all that are happiness" or "all happiness").
Similarly, the dynamic verb {tθoc} ("burn") becomes the stative {djʊ tθoc} ("to
be burning" or "to be a fire"), which can then be instantiated: {za djʊ tθoc}
("a fire").

## Determiners

The **universal distributive determiner** {su} expresses that its immediately
enclosing clause holds true individually for each instance of its complement,
functioning similarly to "each" or "every". This determiner is unique in that it
only binds pronouns for the duration of the containing clause.

<gloss>
{je} | {rʊ} | {su} | {vɪx}    | {fxe} | {cay}
ᴵᴺᴰ  | ᴾ    | each | language | ʜᴀʙ   | change
Every language changes.
</gloss>

TODO: Examples illustrating why the universal and existential determiners vary
in binding scope as they do. "Every man X has a horse Y; Y is a good horse"
doesn't make sense. "A man X has a horse Y; Y is a good horse" does.


The **universal collective determiner** {wo} could be translated as "all". It is
similar to the universal distributive determiner except that it quantifies its
complement collectively rather than individually. TODO: practical difference?


Generally, bound pronouns remain bound until rebound to a new referent. However,
any pronouns bound using the universal determiner {u}—or in any subsequent
quantified term within the same clause—become unbound at the end of the current
clause.

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
