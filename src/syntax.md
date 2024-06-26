# Syntax

Najan has the following word classes:

- Verbs ({fyas})
- Determiners ({qa saθ dan}, _means of instantiation_)
- Pronouns ({nʊ zax kʊv}, _borrowed identity_)
- Prepositions ({qa qac dan}, _means of relation_)
- Conjunctions ({qa cob vom}, _logical operator_)
- Particles ({lʊg})

Najan has no nouns, adverbs, or adjectives. Verb phrases, pronouns, and
[determiner phrases](./determiner-phrases.md) can all function as noun phrases.
(A verb phrase may be used as a verbal noun phrase without any inflection or
other marking.) Prepositional phrases fill the roles of adverbs and adjectives.

## Formal Grammar

Najan is a deterministic context-free language whose formal grammar can be
expressed in extended Backus-Naur form as follows:

<!-- prettier-ignore -->
> <lhs>**clause** ⇒</lhs> ⟨**mood ᴘᴛᴄʟ**⟩ ⟨**VP**⟩
> <br>
> <lhs>|</lhs> ⟨**PP**⟩ ⟨**clause**⟩
> <br>
> <lhs>|</lhs> ⟨**restrictive clause**⟩ ⟨**clause**⟩
> <br>
> <lhs>|</lhs> ⟨**non-restrictive clause**⟩ ⟨**clause**⟩
>
> <lhs>**ᴘʀᴇᴘ phrase (PP)** ⇒</lhs> ⟨**ᴘʀᴇᴘ**⟩ ⟨**NP**⟩
>
> <lhs>**restrictive clause** ⇒</lhs> {nʊ} ⟨**VP**⟩
>
> <lhs>**non-restrictive clause** ⇒</lhs> {vew} ⟨**VP**⟩
>
> <lhs>**verb phrase (VP)** ⇒</lhs> [ ⟨**aspect ᴘᴛᴄʟ**⟩ ] ⟨**verb**⟩
> <br>
> <lhs>|</lhs> {ma} ⟨**VP**⟩
> <br>
> <lhs>|</lhs> ⟨**PP**⟩ ⟨**VP**⟩
> <br>
> <lhs>|</lhs> ⟨**restrictive clause**⟩ ⟨**VP**⟩
> <br>
> <lhs>|</lhs> ⟨**non-restrictive clause**⟩ ⟨**VP**⟩
> <br>
> <lhs>|</lhs> {txa} ⟨**VP**⟩
> <br>
> <lhs>|</lhs> ⟨**ᴄᴏɴᴊ**⟩ ⟨**VP**⟩ ⟨**VP**⟩
>
> <lhs>**noun phrase (NP)** ⇒</lhs> ⟨**VP**⟩ | ⟨**restricted NP**⟩
>
> <lhs>**restricted NP** ⇒</lhs> ⟨**ᴘʀᴏ**⟩
> <br>
> <lhs>|</lhs> ⟨**ᴅᴇᴛ phrase**⟩
> <br>
> <lhs>|</lhs> ⟨**quotation**⟩
> <br>
> <lhs>|</lhs> ⟨**non-restrictive clause**⟩ ⟨**restricted NP**⟩
> <br>
> <lhs>|</lhs> {txa} ⟨**restricted NP**⟩
> <br>
> <lhs>|</lhs> ⟨**ᴄᴏɴᴊ**⟩ ⟨**restricted NP**⟩ ⟨**restricted NP**⟩
>
> <lhs>**ᴅᴇᴛ phrase** ⇒</lhs> ⟨**ᴅᴇᴛ**⟩ [ ⟨**ᴘʀᴏ**⟩ ] ⟨**VP**⟩
>
> <lhs>**quotation** ⇒</lhs> {tca} ⟨**quoted word**⟩+ {tca}
>
> <lhs>**quoted word** ⇒</lhs> ⟨**word**⟩ - {tca} - {tsa} | {tsa tca} | {tsa tsa}
