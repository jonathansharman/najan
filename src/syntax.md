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
> <span class="lhs">**clause** ⇒</span> ⟨**mood ᴘᴛᴄʟ**⟩ ⟨**VP**⟩
> <br>
> <span class="lhs">|</span> ⟨**PP**⟩ ⟨**clause**⟩
> <br>
> <span class="lhs">|</span> ⟨**restrictive clause**⟩ ⟨**clause**⟩
> <br>
> <span class="lhs">|</span> ⟨**non-restrictive clause**⟩ ⟨**clause**⟩
>
> <span class="lhs">**ᴘʀᴇᴘ phrase (PP)** ⇒</span> ⟨**ᴘʀᴇᴘ**⟩ ⟨**NP**⟩
>
> <span class="lhs">**restrictive clause** ⇒</span> {nʊ} ⟨**VP**⟩
>
> <span class="lhs">**non-restrictive clause** ⇒</span> {vew} ⟨**VP**⟩
>
> <span class="lhs">**verb phrase (VP)** ⇒</span> [ ⟨**aspect ᴘᴛᴄʟ**⟩ ] ⟨**verb**⟩
> <br>
> <span class="lhs">|</span> {ma} ⟨**VP**⟩
> <br>
> <span class="lhs">|</span> ⟨**PP**⟩ ⟨**VP**⟩
> <br>
> <span class="lhs">|</span> ⟨**restrictive clause**⟩ ⟨**VP**⟩
> <br>
> <span class="lhs">|</span> ⟨**non-restrictive clause**⟩ ⟨**VP**⟩
> <br>
> <span class="lhs">|</span> {txa} ⟨**VP**⟩
> <br>
> <span class="lhs">|</span> ⟨**ᴄᴏɴᴊ**⟩ ⟨**VP**⟩ ⟨**VP**⟩
>
> <span class="lhs">**noun phrase (NP)** ⇒</span> ⟨**VP**⟩ | ⟨**restricted NP**⟩
>
> <span class="lhs">**restricted NP** ⇒</span> ⟨**ᴘʀᴏ**⟩
> <br>
> <span class="lhs">|</span> ⟨**ᴅᴇᴛ phrase**⟩
> <br>
> <span class="lhs">|</span> ⟨**quotation**⟩
> <br>
> <span class="lhs">|</span> ⟨**non-restrictive clause**⟩ ⟨**restricted NP**⟩
> <br>
> <span class="lhs">|</span> {txa} ⟨**restricted NP**⟩
> <br>
> <span class="lhs">|</span> ⟨**ᴄᴏɴᴊ**⟩ ⟨**restricted NP**⟩ ⟨**restricted NP**⟩
>
> <span class="lhs">**ᴅᴇᴛ phrase** ⇒</span> ⟨**ᴅᴇᴛ**⟩ [ ⟨**ᴘʀᴏ**⟩ ] ⟨**VP**⟩
>
> <span class="lhs">**quotation** ⇒</span> {tca} ⟨**quoted word**⟩+ {tca}
>
> <span class="lhs">**quoted word** ⇒</span> ⟨**word**⟩ - {tca} - {tsa} | {tsa tca} | {tsa tsa}
