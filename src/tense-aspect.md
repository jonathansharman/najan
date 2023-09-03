# Tense & Aspect

Tense specifies *when* along a timeline a verb phrase occurs (e.g. past,
present, future), whereas aspect characterizes *how* the verb phrase extends
over time. In Najan, aspect is marked syntactically with a (possibly null)
particle immediately preceding the verb or verb phrase. In contrast, tense has
no direct syntactic representation, so it must be marked explicitly with
[temporal arguments](./case.md#spatio-temporal-cases), [inferred from the
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
be in past tense. For example, {je ci ko θe tcet} is more likely to mean "I ate"
(simple past) or "I have eaten" (past perfect) than "I will eat" (simple
future).

The **inchoative** (ɪɴᴄʜ) aspect views a verb as beginning, as in {je dje sic},
"[he] starts to cry". Inversely, the **cessative** (ᴄᴇss) aspect views a verb as
ending: {je vat sic}, "[he] stops crying". By default, verbs in these aspects
are in present tense.

The **habitual** (ʜᴀʙ) aspect views a verb as occurring regularly over some
period of time, centered on the present by default. If {je ci to vɪg} is in
present tense, it means "you are going", and the corresponding habitual phrase,
{je ci to fxe vɪg}, means "you go [regularly]". If we instead assume past tense,
then they translate respectively to "you were going" and either "you would go"
or "you used to go".

The **gnomic** (ɢɴᴏ) aspect views a verb as a general truth with little or no
temporal structure. As such, the concept of tense is usually inapplicable.
Gnomic verb phrases often feature generic [determiner
phrases](./determiner-phrases.md) as arguments and express an aphorism, a
mathematical truth, etc.

<gloss>
je  | ŋɪ  | ðu  | zʊn | qa  | ðu  | djʊr | hon | dzul
ɪɴᴅ | sʙᴊ | ɢɴʀ | two | ɢᴇɴ | ɢɴʀ | one  | ɢɴᴏ | is-greater-than
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
je  | lɪ  | ŋodl     | ci | ko | θe  | vɪg
ɪɴᴅ | ʟᴏᴄ | tomorrow | ᴀ  | I  | ᴘғᴠ | go
I will go tomorrow.
</gloss>

At this point I have established tomorrow (i.e. the future) as the time period
under discussion, using a [temporal argument](./case.md#spatio-temporal-cases).
Suppose I then say {je ci ko θe tɪz}. In isolation, since this sentence is in
perfective aspect, one might interpret it to mean "I have spoken" (present
perfect) or "I spoke" (simple past). However, since the context has already been
established as in the future (thanks to the locative argument), one should
instead translate the second clause as "I will speak" or "I will have spoken"
sometime after I have gone tomorrow.
