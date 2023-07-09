# Syntax

Najan is a deterministic context-free language whose grammar can be expressed in
extended Backus-Naur form as follows:

> <lhs>**clause** ⇒</lhs> ⟨**mood ᴘᴛᴄʟ**⟩ ⟨**VP**⟩
>
> <lhs>**verb phrase (VP)** ⇒</lhs> [ ⟨**aspect ᴘᴛᴄʟ**⟩ ] ⟨**dynamic verb**⟩
> <br>
> <lhs>|</lhs> ⟨**stative VP**⟩
> <br>
> <lhs>|</lhs> ⟨**ᴘʀᴇᴘ**⟩ ⟨**NP**⟩ ⟨**VP**⟩
> <br>
> <lhs>|</lhs> ⟨**ᴄᴏɴᴊ**⟩ ⟨**VP**⟩ ⟨**VP**⟩
>
> <lhs>**stative VP** ⇒</lhs> [ ⟨**aspect ᴘᴛᴄʟ**⟩ ] ⟨**stative verb**⟩
> <br>
> <lhs>|</lhs> "{djʊ}" ⟨**VP**⟩
> <br>
> <lhs>|</lhs> "{vʊ}" ⟨**case ᴘʀᴇᴘ**⟩ ⟨**VP**⟩
> <br>
> <lhs>|</lhs> "{ma}" ⟨**stative VP**⟩
> <br>
> <lhs>|</lhs> ⟨**ᴘʀᴇᴘ**⟩ ⟨**NP**⟩ ⟨**stative VP**⟩
> <br>
> <lhs>|</lhs> ⟨**ᴄᴏɴᴊ**⟩ ⟨**stative VP**⟩ ⟨**stative VP**⟩
>
> <lhs>**noun phrase (NP)** ⇒</lhs> ⟨**VP**⟩ | ⟨**ᴘʀᴏ**⟩ | ⟨**ᴅᴇᴛ phrase**⟩ | ⟨**quotation**⟩ | ⟨**ᴄᴏɴᴊ**⟩ ⟨**NP**⟩ ⟨**NP**⟩
>
> <lhs>**ᴅᴇᴛ phrase** ⇒</lhs> ⟨**ᴅᴇᴛ**⟩ [ ⟨**ᴘʀᴏ**⟩ ] ⟨**stative VP**⟩
>
> <lhs>**quotation** ⇒</lhs> "{tca}" ⟨**quoted word**⟩ "{tca}"
>
> <lhs>**quoted word** ⇒</lhs> ⟨**word**⟩ - "{tca}" - "{tsa}" | "{tsa tca}" | "{tsa tsa}"
