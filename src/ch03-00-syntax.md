# Syntax

Najan is a deterministic context-free language whose grammar can be expressed in
extended Backus-Naur form as follows:

> <lhs>**clause** ⇒</lhs> [ ⟨**mood ᴘᴛᴄʟ**⟩ ] ⟨**VP**⟩
>
> <lhs>**verb phrase (VP)** ⇒</lhs> [ ⟨**aspect ᴘᴛᴄʟ**⟩ ] ⟨**verb**⟩ | ⟨**ᴘʀᴇᴘ**⟩ ⟨**NP**⟩ ⟨**VP**⟩ | ⟨**ᴄᴏɴᴊ**⟩ ⟨**VP**⟩ ⟨**VP**⟩
>
> <lhs>**noun phrase (NP)** ⇒</lhs> ⟨**ᴘʀᴏ**⟩ | ⟨**ᴅᴇᴛ phrase**⟩ | ⟨**quotation**⟩ | ⟨**ᴄᴏɴᴊ**⟩ ⟨**NP**⟩ ⟨**NP**⟩
>
> <lhs>**ᴅᴇᴛ phrase** ⇒</lhs> ⟨**ᴅᴇᴛ**⟩ [ ⟨**ᴘʀᴏ**⟩ ] ⟨**VP**⟩
>
> <lhs>**quotation** ⇒</lhs> "<naj>tca</naj> ⟨tca⟩" ⟨**quoted word**⟩ "<naj>tca</naj> ⟨tca⟩"
>
> <lhs>**quoted word** ⇒</lhs> ⟨**word**⟩ - "<naj>tca</naj> ⟨tca⟩" - "<naj>tsa</naj> ⟨tsa⟩" | "<naj>tsa tca</naj> ⟨tsa tca⟩" | "<naj>tsa tsa</naj> ⟨tsa tsa⟩"

## Quotation

A quotation begins and ends with the particle <naj>tca</naj> ⟨tca⟩ and functions
as a noun signifying the contained words themselves. The particle <naj>tsa</naj>
⟨tsa⟩ is used within a quotation just before <naj>tca</naj> ⟨tca⟩ or another
<naj>tsa</naj> ⟨tsa⟩ to indicate that the second particle should be interpreted
as part of the quotation, not as a particle within the sentence containing the
quotation.
