export type TimeTableType = {
  id: number;
  name: string;
  owner: string;
  events: EventType[];
};
export type EventType = {
  name: string;
  starts_on: Date;
  start_time: Time;
  ends_time: Time;
  where: string;
};
export type Time = {
  hour: number;
  minute: number;
};
// Returns true or false depending on if the TimeTable has something happening today
export type Event = {
  name: string;
  where: string;
  start: Date;
  end: Date;
};
export function getNextEvent(timeTable: TimeTableType): Event {
  const today = new Date();
  if (timeTable.events.length === 0) {
    return {
      name: "Nothing",
      where: "Nowhere",
      start: today,
      end: today,
    };
  }
  // TODO Sort ignoring events that have happened already
  // Pending @NickAcPT's implementation of the backend
  timeTable.events.sort((a, b) => {
    return a.starts_on.getTime() - b.starts_on.getTime();
  });
  const next = timeTable.events[0];
  return {
    name: next.name,
    where: next.where,
    start: new Date(
      next.starts_on.getFullYear(),
      next.starts_on.getMonth(),
      next.starts_on.getDate(),
      next.start_time.hour,
      next.start_time.minute
    ),
    end: new Date(
      next.starts_on.getFullYear(),
      next.starts_on.getMonth(),
      next.starts_on.getDate(),
      next.ends_time.hour,
      next.ends_time.minute
    ),
  };
}
