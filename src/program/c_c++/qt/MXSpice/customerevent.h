#ifndef CUSTOMEREVENT_H
#define CUSTOMEREVENT_H

#include <QString>
#include <QEvent>

class CustomerEvent : public QEvent
{
public:
    CustomerEvent(QString value = "");
    static Type eventType();
    QString getValueString(void);

private:
    static Type m_eventType;
    QString m_string;
};

#endif // CUSTOMEREVENT_H
