#ifndef SPICEREPORTER_H
#define SPICEREPORTER_H

#include <QObject>

class SpiceReporter : public QObject
{
    Q_OBJECT
public:
    explicit SpiceReporter(QObject *parent = nullptr);

    void OnStateChange(bool exited);

private:
    QObject *m_parent;
};

#endif // SPICEREPORTER_H
