# Syntax

Najan is a deterministic context-free language whose grammar can be expressed in
extended Backus-Naur form as follows:

> <lhs>**clause** ⇒</lhs> [ ⟨**mood ᴘᴛᴄʟ**⟩ ] ⟨**VP**⟩
>
> <lhs>**verb phrase (VP)** ⇒</lhs> [ ⟨**aspect ᴘᴛᴄʟ**⟩ ] ⟨**verb**⟩
> <br>
> <lhs>|</lhs> ⟨**stativized VP**⟩
> <br>
> <lhs>|</lhs> ⟨**ᴘʀᴇᴘ**⟩ ⟨**NP**⟩ ⟨**VP**⟩
> <br>
> <lhs>|</lhs> ⟨**ᴄᴏɴᴊ**⟩ ⟨**VP**⟩ ⟨**VP**⟩
>
> <lhs>**stativized VP** ⇒</lhs> "<naj>djʊ</naj> ⟨djʊ⟩" ⟨**VP**⟩ | "<naj>vʊ</naj> ⟨vʊ⟩" ⟨**ᴘʀᴇᴘ**⟩ ⟨**VP**⟩
>
> <lhs>**noun phrase (NP)** ⇒</lhs> ⟨**VP**⟩ | ⟨**ᴘʀᴏ**⟩ | ⟨**ᴅᴇᴛ phrase**⟩ | ⟨**quotation**⟩ | ⟨**ᴄᴏɴᴊ**⟩ ⟨**NP**⟩ ⟨**NP**⟩
>
> <lhs>**ᴅᴇᴛ phrase** ⇒</lhs> ⟨**ᴅᴇᴛ**⟩ [ ⟨**ᴘʀᴏ**⟩ ] ⟨**VP**⟩
>
> <lhs>**quotation** ⇒</lhs> "<naj>tca</naj> ⟨tca⟩" ⟨**quoted word**⟩ "<naj>tca</naj> ⟨tca⟩"
>
> <lhs>**quoted word** ⇒</lhs> ⟨**word**⟩ - "<naj>tca</naj> ⟨tca⟩" - "<naj>tsa</naj> ⟨tsa⟩" | "<naj>tsa tca</naj> ⟨tsa tca⟩" | "<naj>tsa tsa</naj> ⟨tsa tsa⟩"
