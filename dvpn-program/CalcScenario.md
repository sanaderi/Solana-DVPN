# We have an accountant Account include:

## Concept
1. **Server List**
2. **Server Total active remain => is $STAR**
3. **Client Total active remain => is $CTAR**
4. **Total Income => is $TI (Calc on cliam time)**

## Every server save its ativity in own account => $SA
## Asset in PDA => $APDA

### How calc Earn for every server:

$CURRENT_PAID_PRICE=($APDA/$CTAR)
$TOTAL_INCOME=$CURRENT_TIMESTAMPE-$LAST_UPDATE_TIMESTAMP*$CURRENT_PAID_PRICE
$CLAIMABLE=$TOTAL_INCOME/$STAR*$SA

### Example
Server Total active remain ($STAR)=27000s
Your server activity ($SA)=5200s
Client Total active remain ($CTAA)=48000s
PDA Asset ($APDA) = 50SOL

$CURRENT_PAID_PRICE= 50/48000
$TOTAL_INCOME=17800000-122000000*$CURRENT_PAID_PRICE
$CLAIMABLE=$TOTAL_INCOME/27000*5200
