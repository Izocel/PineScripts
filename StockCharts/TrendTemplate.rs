// Mark Minervini's Trend Template - Complete 8 Criteria for Stage 2 Uptrend:
// 'Think & Trade Like a Champion', by Mark Minervini gave his famous Trend Template Criteria
// A stock must meet the following 8 criterias to be deemed in a confirmed Stage 2 uptrend:

[[Exchange is ICE] or [Exchange is CME] or [Exchange is CSE] or
[Exchange is NEO] or [Exchange is TSE] or [Exchange is TSXV] or [Exchange is NYSE] or
[Exchange is NASD] or [Exchange is AMEX]]

and [EPS > 0]
and [SCTR >= 80]
and [Market Cap > 2000]
and [Volume > EMA(12,Volume)]

// 1. The current stock price is above both the 150-day (30-week) and the 200-day (40-week) moving average price lines.
and [Close >= EMA(150, Close)]
and [Close >= EMA(200, Close)]

// 2. The 150-day moving average is above the 200-day moving average.
and [EMA(150, Close) >= EMA(200, Close)]

// 3. The 200-day moving average line is trending up for at least 1 month (preferably 4-5 months minimum in most cases).
and [Slope(260,SMA(200,Close)) > 0]

// 4. The 50-day (10-week) moving average is above both the 150-day and 200-day moving averages.
and [EMA(50, Close) >= EMA(150, Close)]
and [EMA(50, Close) >= EMA(200, Close)]

// 5. The current stock price is above the 50-day moving average.
and [Close >= EMA(50, Close)]

// 6. The current stock price is at least 25% above its 52-week low (30% as per his book 'Trade Like a Stock Market Wizard').
// 7. The current stock price is within at least 25% of its 52-week high (the closer to a new high the better).
and [Close >= 1.25 * Min(260, Close)]
and [Close >= 0.75 * Max(260, Close)]

Rank by SCTR
