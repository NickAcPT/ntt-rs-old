// Returns true or false depending on if the TimeTable has something happening today
import type { EnhancedTimeTable } from "@/types/backend";
import { Weekday } from "@/types/types";

export type Event = {
  name: string;
  description: string;
  start: Date;
  end: Date;
};
export type Calendar = {
  events: Event[];
};
export function buildCalendar(
  timeTable: EnhancedTimeTable,
  start: Date,
  end: Date
): Calendar {
  if (timeTable.entries.length === 0) {
    return {
      events: [],
    };
  }
  // Filters out events that have already happened
  // Find the next event
  const events: Array<Event> = [];
  timeTable.entries.forEach((entry) => {
    if (entry.start_date < start) {
      return;
    }
    if (entry.start_date > end) {
      return;
    }
    const startMonday = new Date(
      start.setDate(
        start.getDate() - start.getDay() + (start.getDay() == 0 ? -6 : 1)
      )
    );
    if (entry.entry_type === "OneTime") {
      events.push({
        name: "PENDING",
        description: "PENDING",
        start: entry.start_time,
        end: entry.end_time,
      });
    } else if (entry.recurring_event) {
      if (entry.recurring_event.repeats_every.type === "Weekly") {
        const monday = new Date(startMonday);
        while (monday < end) {
          const days: Weekday[] = entry.recurring_event?.repeats_every
            .content as Weekday[];
          days.forEach((day) => {
            const start = new Date(monday);
            start.setDate(start.getDate() + day);
            start.setHours(entry.start_time.getHours());
            start.setMinutes(entry.start_time.getMinutes());
            const end = new Date(start);
            end.setHours(entry.end_time.getHours());
            end.setMinutes(entry.end_time.getMinutes());

            if (day === monday.getDay()) {
              const date = new Date(monday);
              date.setDate(monday.getDate() + day);
              events.push({
                name: "PENDING",
                description: "PENDING",
                start: start,
                end: end,
              });
            }
          });
          monday.setDate(monday.getDate() + 7);
        }
      }
    }
  });
  events.sort((a, b) => a.start.getTime() - b.start.getTime());
  return {
    events: events,
  };
}
