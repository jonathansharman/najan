# Tense & Aspect

Tense specifies _when_ along a timeline a verb phrase occurs (e.g. past,
present, future), whereas aspect characterizes _how_ the verb phrase extends
over time. In Najan, aspect is marked syntactically with a (possibly null)
particle immediately preceding the verb or verb phrase. In contrast, tense has
no direct syntactic representation, so it must be marked explicitly with
[temporal arguments](./time-and-place.md), [inferred from the
surrounding context](./tense-aspect.md#inferring-tense-from-context), or else
assumed from the verb's aspect.

| Aspect     | Gloss | Particle |
| :--------- | :---- | :------- |
| Continuous | ᴄᴏɴᴛ  | ---      |
| Perfective | ᴘғᴠ   | {θe}     |
| Inchoative | ɪɴᴄʜ  | {dje}    |
| Cessative  | ᴄᴇss  | {vat}    |
| Habitual   | ʜᴀʙ   | {fxe}    |
| Gnomic     | ɢɴᴏ   | {hon}    |

The **continuous** (ᴄᴏɴᴛ) aspect views a verb as ongoing or in progress. It can
apply to both stative and dynamic verbs (Najan does not distinguish between
continuous and progressive aspects), and it is the default aspect, in the
absence of an aspect particle. Without temporal arguments or additional context,
a continuous verb phrase may be assumed to be in present tense.

The **perfective** (ᴘғᴠ) aspect views a verb in its entirety, as a complete
action or state. Without other clues, a perfective verb phrase is most likely to
be in past tense. For example, {je ci ko θe tcet} is more likely to mean _I ate_
(simple past) or _I have eaten_ (past perfect) than _I will eat_ (simple
future).

The **inchoative** (ɪɴᴄʜ) aspect views a verb as beginning, as in {je dje sic},
_[he] starts to cry_. Inversely, the **cessative** (ᴄᴇss) aspect views a verb as
ending: {je vat sic}, _[he] stops crying_. By default, verbs in these aspects
are in present tense.

The **habitual** (ʜᴀʙ) aspect views a verb as occurring regularly over some
period of time, centered on the present by default. If {je ci to vɪg} is in
present tense, it means _you are going_, and the corresponding habitual phrase,
{je ci to fxe vɪg}, means _you go [regularly]_. If we instead assume past tense,
then they translate respectively to _you were going_ and either _you would go_
or _you used to go_.

The **gnomic** (ɢɴᴏ) aspect views a verb as a general truth with little or no
temporal structure. As such, the concept of tense is usually inapplicable.
Gnomic verb phrases often feature generic [determiner
phrases](./determiner-phrases.md) as arguments and express an aphorism, a
mathematical truth, etc.

<gloss>
ŋo  | ŋɪ  | ðu  | zʊn | qa  | ðu  | djʊr | hon | dzul
ᴅᴇᴅ | sʙᴊ | ɢɴʀ | two | ɢᴇɴ | ɢɴʀ | one  | ɢɴᴏ | is-greater-than
Two is greater than one. (Things of size two are greater than things of size one.)
</gloss>

For a stative gnomic verb phrase, such as in the preceding example, the
distinction from the continuous aspect is subtle. Whereas the given gnomic
version expresses a general mathematical fact, the continuous version (without
{hon}) would leave open the possibility that two might not have always been or
might not always be greater than one.

## Inferring Tense from Context

As described above, aspect markers can also carry a small amount of tense
information. For example, perfective aspect often implies past tense. However,
these associations are secondary to context. Suppose I first say:

<gloss>
lay | lɪ  | ŋodl     | ci | ko | θe  | vɪg
ᴄᴍs | ʟᴏᴄ | tomorrow | ᴀ  | I  | ᴘғᴠ | go
I will go tomorrow.
</gloss>

At this point I have established tomorrow (i.e. the future) as the time period
under discussion, using a [temporal argument](./case.md#spatio-temporal-cases).
Suppose I then say {je ci ko θe tɪz}. In isolation, since this sentence is in
perfective aspect, one might interpret it to mean _I have spoken_ (present
perfect) or _I spoke_ (simple past). However, since the context has already been
established as in the future (thanks to the locative argument), one should
instead translate the second clause as _I will speak_ or _I will have spoken_
sometime after I have gone tomorrow.
