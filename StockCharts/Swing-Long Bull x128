// For the last intraday update:
// [[EXCHANGE IS LSE] OR [EXCHANGE IS ICE] OR [EXCHANGE IS CME] OR [EXCHANGE IS CSE] OR [EXCHANGE IS NEO] OR [EXCHANGE IS TSE] OR [EXCHANGE IS TSXV] OR[EXCHANGE IS NYSE] OR [EXCHANGE IS NASD] OR [EXCHANGE IS AMEX]] AND [MARKET CAP > 2,000] AND [SCTR >= 60] AND [RSI(14) >= 64.00] AND [SMA(24, CLOSE) <= SMA(128)] AND [MACD LINE(12,26,9) >= MACD SIGNAL(12,26,9)] AND [CLOSE >= ICHIMOKU CLOUD TOP(9,26,52)] AND [CLOSE >= ICHIMOKU BASE LINE(9,26,52)] AND [CLOSE >= ICHIMOKU CONVERSION LINE(9,26,52)] AND [ICHIMOKU CONVERSION LINE(9,26,52) >= ICHIMOKU BASE LINE(9,26,52)]

[[exchange is ICE] or [exchange is CME] or [exchange is CSE] or
[exchange is NEO] or [exchange is TSE] or [exchange is TSXV] or[exchange is NYSE] or
[exchange is NASD] or [exchange is AMEX]]

and [market cap > 2,000] and [SCTR >= 60]

and [RSI(14) >= 64.00]
and [SMA(24, close) <= SMA(128)]
and [MACD Line(12,26,9) >= MACD Signal(12,26,9)]
and [close >= Ichimoku Cloud Top(9,26,52)]
and [close >= Ichimoku Base Line(9,26,52)]
and [close >= Ichimoku Conversion Line(9,26,52)]
and [Ichimoku Conversion Line(9,26,52) >= Ichimoku Base Line(9,26,52)]

// DESCRIPTION:
// This is an aggressive swing trading strategy designed to identify strong bullish momentum
// in large-cap stocks using a combination of traditional and Japanese technical indicators.
//
// KEY COMPONENTS:
// - Market Cap Filter: Only considers stocks with market cap > 2 billion (large-cap focus)
// - SCTR Filter: Requires SCTR >= 60 to focus on relatively strong stocks
// - RSI(14): Must be >= 64 (strong momentum, not quite overbought)
// - Moving Average Setup: SMA(24) <= SMA(128) (shorter MA below longer MA for trend confirmation)
// - MACD Bullish Signal: MACD Line >= MACD Signal (positive momentum crossover)
// - Ichimoku Cloud Analysis: Price must be above all key Ichimoku components
//   * Price > Cloud Top (bullish trend confirmation)
//   * Price > Base Line (medium-term bullish bias)
//   * Price > Conversion Line (short-term bullish bias)
//   * Conversion Line > Base Line (additional bullish signal)
//
// STRATEGY LOGIC:
// The "x128" designation refers to the 128-period SMA being used as a long-term trend filter.
// This strategy identifies stocks in strong uptrends with momentum acceleration,
// confirmed by both Western (RSI, MACD, SMA) and Eastern (Ichimoku) technical indicators.
// The Ichimoku component ensures the stock is in a clear bullish phase across multiple timeframes.
//
// ENTRY CONDITIONS: All conditions must be met simultaneously
// TIMEFRAME: Best suited for swing trading (3-10 day holds)
// RISK LEVEL: Medium-High - focuses on momentum continuation rather than reversal
// UPDATE FREQUENCY: Tracks signals from 12 trading days before last intraday update
