// Returns true or false depending on if the TimeTable has something happening today
import type { EnhancedTimeTable } from "@/types/backend";

export type Event = {
  name: string;
  where: string;
  start: Date;
  end: Date;
};
export function getNextEvent(timeTable: EnhancedTimeTable): Event {
  const now = new Date();
  // Get the date of the Monday of the current week
  const monday = new Date(
    now.setDate(now.getDate() - now.getDay() + (now.getDay() == 0 ? -6 : 1))
  );

  if (timeTable.entries.length === 0) {
    return {
      name: "Nothing",
      where: "Nowhere",
      start: now,
      end: now,
    };
  }
  // Filters out events that have already happened
  let recurringExists = false;
  timeTable.entries.filter((entry) => {
    if (entry.entry_type === "OneTime") {
      return entry.start_time > now;
    } else if (entry.recurring_event) {
      recurringExists = true;
      return entry.recurring_event.end_date >= now;
    } else {
      console.error("Recurring event is undefined");
    }
  });
  if (!recurringExists) {
    timeTable.entries.sort((a, b) => {
      return a.start_time.getTime() - b.start_time.getTime();
    });
    return {
      name: "PENDING",
      where: "PENDING",
      start: timeTable.entries[0].start_time,
      end: timeTable.entries[0].end_time,
    };
  }
  // Find the next event
  const events: Array<Event> = [];
  timeTable.entries.forEach((entry) => {
    if (entry.entry_type === "OneTime") {
      events.push({
        name: "PENDING",
        where: "PENDING",
        start: entry.start_time,
        end: entry.end_time,
      });
    } else if (entry.recurring_event) {
      if (entry.recurring_event.repeats_every.type === "Weekly") {
        entry.recurring_event.repeats_every.content.forEach((day) => {
          const eventDate = new Date(monday);
          eventDate.setDate(eventDate.getDate() + day);
          eventDate.setHours(entry.start_time.getHours());
          eventDate.setMinutes(entry.start_time.getMinutes());
          eventDate.setSeconds(entry.start_time.getSeconds());
          events.push({
            name: "PENDING",
            where: "PENDING",
            start: eventDate,
            end: new Date(eventDate.getTime() + entry.duration * 60000),
          });
        });
      }
    }
  });
  events.sort((a, b) => a.start.getTime() - b.start.getTime());
  return events[0];
}
