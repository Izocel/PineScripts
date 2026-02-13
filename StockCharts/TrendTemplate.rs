// For the last intraday update:
// [[EXCHANGE IS ICE] OR [EXCHANGE IS CME] OR [EXCHANGE IS CSE] OR [EXCHANGE IS NEO] OR [EXCHANGE IS TSE] OR [EXCHANGE IS TSXV] OR[EXCHANGE IS NYSE] OR [EXCHANGE IS NASD] OR [EXCHANGE IS AMEX]] AND [MARKET CAP > 2,000] AND [CLOSE >= EMA(150, CLOSE)] AND [CLOSE >= EMA(200, CLOSE)] AND [EMA(150, CLOSE) >= EMA(200, CLOSE)] AND [EMA(200, CLOSE) >= 1 MONTH'S AGO EMA(200, CLOSE)] AND [EMA(50, CLOSE) >= EMA(150, CLOSE)] AND [EMA(50, CLOSE) >= EMA(200, CLOSE)] AND [LOW >= YESTERDAY'S MIN(253,LOW) * 1.25] AND [HIGH <= YESTERDAY'S MAX(253,HIGH) * 0.75]

// 'Think & Trade Like a Champion', by Mark Minervini gave his famous Trend Template Criteria
// A stock must meet the following 8 criterias to be deemed in a confirmed Stage 2 uptrend:

[[exchange is ICE] or [exchange is CME] or [exchange is CSE] or
[exchange is NEO] or [exchange is TSE] or [exchange is TSXV] or[exchange is NYSE] or
[exchange is NASD] or [exchange is AMEX]]

and [market cap > 2,000] //and [SCTR >= 70]

// 1. The current stock price is above both the 150-day (30-week) and the 200-day (40-week) moving average price lines.
and [close >= EMA(150, close)]
and [close >= EMA(200, close)]

// 2. The 150-day moving average is above the 200-day moving average.
and [EMA(150, close) >= EMA(200, close)]

// 3. The 200-day moving average line is trending up for at least 1 month (preferably 4,5 months minimum in most cases).
and [EMA(200, close) >= 1 month's ago EMA(200, close)]

// 4. The 50-day (10-week) moving average is above both the 150-day and 200-day moving averages.
and [EMA(50, close) >= EMA(150, close)]
and [EMA(50, close) >= EMA(200, close)]


// 6. The current stock price is at least 25% above its 52-week low (30% as per his book 'Trade Like a Stock Market Wizard').
// 7. The current stock price is within at least 25% of its 52-week high (the closer to a new high the better).
and [low >= yesterday's min(253,low) * 1.25]
and [high <= yesterday's max(253,high) * 0.75]

// 8. The Relative Strength ranking (RS ranking), as reported in Investor's Business Daily, is no less than 70.
//TODO: Define a qualitative RS ranking filter, as this is not a standard technical indicator...