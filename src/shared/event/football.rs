use crate::shared::db::ToDBRecord;

#[derive(Debug, Clone)]
pub enum Football {
    Winner(Part),
    Winner2(Part, Part),
    Winner2BothToScore(Part, Part),
    WinnerMatchAndGoals(Part),
    Goals(Part),
    Goals2(Part, Part),
    GoalsPlayer(Player, Part),
    ExactGoals(Part),
    ExactGoalsPlayer(Player),
    GoalBothHalves,
    BothGoalBothHalves,
    BothToScore(Part),
    BothToScoreGoals,
    BothToScoreSameHalf,
    BothToScoreOrGoalsOver,
    BothToScoreAtLeast(u32),
    WinnerBothToScore(Part, Part),
    Handicap(Part),
    Corners(Part),
    CornersHandicap(Part),
    CornersPlayer(Player, Part),
    MultiGoals(Part),
    MultiGoalsPlayer(Player),
    DrawNoBet(Part),
    MatchBothToScore,
    Offsides,
    OffsidesPlayer(Player),
    MatchMultiScore,
    Penalty,
    PenaltySeries,
    DoubleChance(Part),
    DoubleChanceH1OrMatch,
    MatchCornerRange,
    HalfWithMoreGoals,
    HalfWithMoreYellowCards,
    WillGetCard,
    DoubleChanceGoalRange,
    FirstGoal(Part),
    FirstGoalMatch,
    FirstGoalMinute,
    FirstGoalMinutePlayer(Player),
    CornerRange(Part),
    CornerRangePlayer(Player, Part),
    MatchScorePlayers,
    MatchCorners(Part),
    MatchCornersHandicap(Part),
    RestProduct,
    WinToNil(Player),
    WinBothHalves(Player),
    WinAtLeastOneHalf(Player),
    ExactScore(Part),
    ScoreBothHalves(Player),
    GoalBeforeMinute,
    NoGoalBeforeMinute,
    DoubleChanceBothToScore,
    WinDiff,
    FirstCorner(Part),
    MatchShotsOnTarget,
    ShotsOnTarget,
    ShotsOnTargetPlayer(Player),
    PlayerShot,
    PlayerShotOnTarget,
    MoreCorners(Part),
    MoreShotsOnTarget,
    MoreYellowCards,
    MoreFouls,
    MatchMoreCorners,
    Minute15,
    Minute30,
    Minute60,
    Minute75,
    PlayerToScore,
    YellowCards(Part),
    YellowCardsPlayer(Player, Part),
    RedCard(Part),
    RedCardPlayer(Player),
    ResultDuringMatch,
    NotResultDuringMatch,
    MatchGoals(Part),
    MatchGoalsPlayer(Player),
    GoalRange,
    SubstituteWillScore,
    WillBeLosingBut,
    Meeting(Part),
    Match(Part),
    ToAdvance,
    AdvanceBy,
    FinaleWinner,
    Shift(Part),
    Fouls,
    FoulsPlayer(Player),
    SuicideGoal,
    Superoffer,
    MatchGoalSum,
}

impl ToDBRecord for Football {
    fn to_db_record(&self) -> Option<String> {
        use Football::*;
        let x = match self {
            Winner(Part::FullTime) => "winner",
            Winner(Part::FirstHalf) => "winner_h1",
            Winner(Part::SecondHalf) => "winner_h2",
            _ => return None,
        };
        Some(format!("football_event:{x}"))
    }
}

#[derive(Debug, Clone)]
pub enum Player {
    P1,
    P2,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Part {
    FullTime,
    FirstHalf,
    SecondHalf,
}

impl std::fmt::Display for Football {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
