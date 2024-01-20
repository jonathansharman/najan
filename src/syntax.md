# Syntax

Najan has the following word classes:

- Stative verbs ({loð}, _state_)
- Dynamic verbs ({djʊ}, _action_)
- Determiners ({qa saθ dan}, _means of instantiation_)
- Pronouns ({nʊ zax kʊv}, _borrowed identity_)
- Prepositions ({qa qac dan}, _means of relation_)
- Conjunctions ({qa cob vom}, _logical operator_)
- Particles ({lʊg}, _particle_)

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
> <lhs>|</lhs> ⟨**ᴘʀᴇᴘ**⟩ ⟨**NP**⟩ ⟨**clause**⟩
>
> <lhs>**verb phrase (VP)** ⇒</lhs> [ ⟨**aspect ᴘᴛᴄʟ**⟩ ] ⟨**dynamic verb**⟩
> <br>
> <lhs>|</lhs> ⟨**stative VP**⟩
> <br>
> <lhs>|</lhs> ⟨**ᴘʀᴇᴘ**⟩ ⟨**NP**⟩ ⟨**VP**⟩
> <br>
> <lhs>|</lhs> "{txa}" ⟨**VP**⟩
> <br>
> <lhs>|</lhs> ⟨**ᴄᴏɴᴊ**⟩ ⟨**VP**⟩ ⟨**VP**⟩
>
> <lhs>**stative VP** ⇒</lhs> [ ⟨**aspect ᴘᴛᴄʟ**⟩ ] ⟨**stative verb**⟩
> <br>
> <lhs>|</lhs> "{ma}" ⟨**stative VP**⟩
> <br>
> <lhs>|</lhs> ⟨**ᴘʀᴇᴘ**⟩ ⟨**NP**⟩ ⟨**stative VP**⟩
> <br>
> <lhs>|</lhs> "{txa}" ⟨**stative VP**⟩
> <br>
> <lhs>|</lhs> ⟨**ᴄᴏɴᴊ**⟩ ⟨**stative VP**⟩ ⟨**stative VP**⟩
>
> <lhs>**noun phrase (NP)** ⇒</lhs> ⟨**VP**⟩
> <br>
> <lhs>|</lhs> ⟨**ᴘʀᴏ**⟩
> <br>
> <lhs>|</lhs> ⟨**ᴅᴇᴛ phrase**⟩
> <br>
> <lhs>|</lhs> ⟨**quotation**⟩
> <br>
> <lhs>|</lhs> "{txa}" ⟨**NP**⟩
> <br>
> <lhs>|</lhs> ⟨**ᴄᴏɴᴊ**⟩ ⟨**NP**⟩ ⟨**NP**⟩
>
> <lhs>**ᴅᴇᴛ phrase** ⇒</lhs> ⟨**ᴅᴇᴛ**⟩ [ ⟨**ᴘʀᴏ**⟩ ] ⟨**stative VP**⟩
>
> <lhs>**quotation** ⇒</lhs> "{tca}" ⟨**quoted word**⟩ "{tca}"
>
> <lhs>**quoted word** ⇒</lhs> ⟨**word**⟩ - "{tca}" - "{tsa}" | "{tsa tca}" | "{tsa tsa}"
