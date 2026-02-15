import { useState, useEffect } from "react";

interface RoundTimerProps {
  initialSeconds?: number;
  playingNow?: number;
}

const RoundTimer = ({ initialSeconds = 60, playingNow = 142 }: RoundTimerProps) => {
  const [seconds, setSeconds] = useState(initialSeconds);

  // Sync state with prop changes and handle countdown logic
  useEffect(() => {
    setSeconds(initialSeconds);

    const interval = window.setInterval(() => {
      setSeconds((prev) => {
        if (prev > 0) {
          return prev - 1;
        }
        // Loop back to start for continuous rounds
        return initialSeconds;
      });
    }, 1000);

    return () => clearInterval(interval);
  }, [initialSeconds]);

  const formatTime = (totalSeconds: number): string => {
    const safeSeconds = Math.max(0, totalSeconds);
    const minutes = Math.floor(safeSeconds / 60);
    const secs = safeSeconds % 60;
    return `${minutes}:${secs.toString().padStart(2, "0")}`;
  };

  const clampedSeconds = Math.max(seconds, 0);
  const progress = initialSeconds > 0 ? (clampedSeconds / initialSeconds) * 100 : 0;
  
  const circumference = 2 * Math.PI * 45;
  const offset = circumference - (progress / 100) * circumference;

  return (
    <div 
      className="bg-white dark:bg-gray-800 p-6 shadow-sm rounded-xl border border-gray-100 dark:border-gray-700"
      role="timer"
      aria-live="polite"
      aria-label="Round countdown timer"
    >
      <div className="flex flex-col items-center justify-center">
        <div className="relative">
          <svg className="w-32 h-32 transform -rotate-90" viewBox="0 0 100 100" aria-hidden="true">
            <circle
              cx="50"
              cy="50"
              r="45"
              stroke="currentColor"
              strokeWidth="8"
              fill="none"
              className="text-gray-200 dark:text-gray-700"
            />
            <circle
              cx="50"
              cy="50"
              r="45"
              stroke="currentColor"
              strokeWidth="8"
              fill="none"
              strokeDasharray={circumference}
              strokeDashoffset={offset}
              strokeLinecap="round"
              className="text-[#2C4BFD] transition-all duration-1000 ease-linear"
            />
          </svg>
          <div className="absolute inset-0 flex items-center justify-center">
            <span className="text-3xl font-bold text-gray-900 dark:text-white">
              {formatTime(seconds)}
            </span>
          </div>
        </div>

        <div className="mt-6 flex items-center gap-2" aria-label={`Current active players: ${playingNow}`}>
          <span className="inline-block w-2 h-2 bg-green-500 rounded-full" aria-hidden="true"></span>
          <span className="text-lg font-semibold text-gray-600 dark:text-gray-300">
            Playing now: {playingNow}
          </span>
        </div>
      </div>
    </div>
  );
};

export default RoundTimer;
