// For the last intraday update:
// [[EXCHANGE IS ICE] OR [EXCHANGE IS CME] OR [EXCHANGE IS CSE] OR [EXCHANGE IS NEO] OR [EXCHANGE IS TSE] OR [EXCHANGE IS TSXV] OR[EXCHANGE IS NYSE] OR [EXCHANGE IS NASD] OR [EXCHANGE IS AMEX]] AND [MARKET CAP > 5,000] AND [SCTR >= 80] AND [RSI(14) >= 64.00] AND [MACD LINE(12,26,9) >= MACD SIGNAL(12,26,9)] AND [CLOSE >= ICHIMOKU BASE LINE(9,26,52)] AND [CLOSE >= ICHIMOKU CONVERSION LINE(9,26,52)] AND [ICHIMOKU CONVERSION LINE(9,26,52) >= ICHIMOKU BASE LINE(9,26,52)]

[[exchange is ICE] or [exchange is CME] or [exchange is CSE] or
[exchange is NEO] or [exchange is TSE] or [exchange is TSXV] or[exchange is NYSE] or
[exchange is NASD] or [exchange is AMEX]]

and [market cap > 5,000] and [SCTR >= 80]

and [RSI(14) >= 64.00]
and [MACD Line(12,26,9) >= MACD Signal(12,26,9)]
and [close >= Ichimoku Base Line(9,26,52)]
and [close >= Ichimoku Conversion Line(9,26,52)]
and [Ichimoku Conversion Line(9,26,52) >= Ichimoku Base Line(9,26,52)]

// DESCRIPTION:
// This is a swing trading strategy focused on identifying strong bullish momentum
// in large-cap stocks using a simplified but effective combination of momentum and trend indicators.
//
// KEY COMPONENTS:
// - Market Cap Filter: Only considers stocks with market cap > 5 billion (large-cap stability)
// - SCTR Filter: Requires SCTR >= 80 to focus on top-performing stocks
// - RSI(14): Must be >= 64 (strong momentum territory without being severely overbought)
// - MACD Bullish Signal: MACD Line >= MACD Signal (confirming positive momentum trend)
// - Ichimoku Trend Confirmation: Price must be above key Ichimoku levels
//   * Price > Base Line (medium-term bullish trend)
//   * Price > Conversion Line (short-term bullish trend)
//   * Conversion Line > Base Line (bullish momentum alignment)
//
// STRATEGY LOGIC:
// This is a streamlined version compared to "Swing-Long Bull x128", focusing on essential
// momentum and trend confirmation without the cloud analysis or SMA filters.
// The strategy targets stocks in clear uptrends with accelerating momentum,
// using the Ichimoku system to confirm trend direction across multiple timeframes.
//
// ENTRY CONDITIONS: All conditions must be met for a valid signal
// TIMEFRAME: Designed for swing trading (2-7 day holds)
// RISK LEVEL: Medium - balances momentum confirmation with trend validation
// UPDATE FREQUENCY: Optimized for intraday updates as noted in header

