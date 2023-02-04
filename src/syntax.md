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
> <lhs>**stativized VP** ⇒</lhs> "{djʊ}" ⟨**VP**⟩ | "{vʊ}" ⟨**ᴘʀᴇᴘ**⟩ ⟨**VP**⟩
>
> <lhs>**noun phrase (NP)** ⇒</lhs> ⟨**VP**⟩ | ⟨**ᴘʀᴏ**⟩ | ⟨**ᴅᴇᴛ phrase**⟩ | ⟨**quotation**⟩ | ⟨**ᴄᴏɴᴊ**⟩ ⟨**NP**⟩ ⟨**NP**⟩
>
> <lhs>**ᴅᴇᴛ phrase** ⇒</lhs> ⟨**ᴅᴇᴛ**⟩ [ ⟨**ᴘʀᴏ**⟩ ] ⟨**VP**⟩
>
> <lhs>**quotation** ⇒</lhs> "{tca}" ⟨**quoted word**⟩ "{tca}"
>
> <lhs>**quoted word** ⇒</lhs> ⟨**word**⟩ - "{tca}" - "{tsa}" | "{tsa tca}" | "{tsa tsa}"
